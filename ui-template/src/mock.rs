// ABOUTME: Mock data for UI sandbox testing
// ABOUTME: Provides fake sessions and messages to demonstrate UI rendering

#![allow(dead_code)]

use chrono::{DateTime, Duration, Utc};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionSource {
    ClaudeCode,
    CodexCli,
}

impl SessionSource {
    pub fn display_name(&self) -> &'static str {
        match self {
            SessionSource::ClaudeCode => "Claude",
            SessionSource::CodexCli => "Codex",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            SessionSource::ClaudeCode => "●",
            SessionSource::CodexCli => "■",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    User,
    Assistant,
}

#[derive(Debug, Clone)]
pub struct Message {
    pub role: Role,
    pub content: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct Session {
    pub id: String,
    pub source: SessionSource,
    pub project_name: String,
    pub cwd: String,
    pub timestamp: DateTime<Utc>,
    pub messages: Vec<Message>,
}

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub session: Session,
    pub snippet: String,
    pub match_spans: Vec<(usize, usize)>,
}

/// Create mock data for UI testing
pub fn create_mock_data() -> Vec<SearchResult> {
    let now = Utc::now();

    vec![
        // Recent Claude conversation with code
        SearchResult {
            session: Session {
                id: "abc123".to_string(),
                source: SessionSource::ClaudeCode,
                project_name: "recall".to_string(),
                cwd: "~/Documents/Github-Work/recall".to_string(),
                timestamp: now - Duration::minutes(5),
                messages: vec![
                    Message {
                        role: Role::User,
                        content: "How do I implement a two-pane layout in ratatui?".to_string(),
                        timestamp: now - Duration::minutes(10),
                    },
                    Message {
                        role: Role::Assistant,
                        content: "To implement a two-pane layout in ratatui, you can use the `Layout` struct with horizontal constraints:\n\n```rust\nlet chunks = Layout::default()\n    .direction(Direction::Horizontal)\n    .constraints([\n        Constraint::Percentage(40),\n        Constraint::Percentage(60),\n    ])\n    .split(area);\n```\n\nThe left pane will be 40% width and the right pane 60%.".to_string(),
                        timestamp: now - Duration::minutes(5),
                    },
                ],
            },
            snippet: "How do I implement a two-pane layout in ratatui?".to_string(),
            match_spans: vec![(26, 34)], // "two-pane"
        },
        // Codex conversation about git
        SearchResult {
            session: Session {
                id: "def456".to_string(),
                source: SessionSource::CodexCli,
                project_name: "horizon-cf".to_string(),
                cwd: "~/Documents/Github-Work/horizon-cf".to_string(),
                timestamp: now - Duration::hours(2),
                messages: vec![
                    Message {
                        role: Role::User,
                        content: "Show me the git history for the auth module".to_string(),
                        timestamp: now - Duration::hours(2) - Duration::minutes(5),
                    },
                    Message {
                        role: Role::Assistant,
                        content: "Here's the recent git history for the auth module:\n\n```\ncommit a1b2c3d - Add JWT token validation\ncommit e4f5g6h - Implement OAuth2 flow\ncommit i7j8k9l - Initial auth scaffolding\n```".to_string(),
                        timestamp: now - Duration::hours(2),
                    },
                ],
            },
            snippet: "Show me the git history for the auth module".to_string(),
            match_spans: vec![(12, 23)], // "git history"
        },
        // Claude conversation about debugging
        SearchResult {
            session: Session {
                id: "ghi789".to_string(),
                source: SessionSource::ClaudeCode,
                project_name: "api-server".to_string(),
                cwd: "~/Projects/api-server".to_string(),
                timestamp: now - Duration::days(1),
                messages: vec![
                    Message {
                        role: Role::User,
                        content: "Why is my async function blocking the event loop?".to_string(),
                        timestamp: now - Duration::days(1) - Duration::minutes(15),
                    },
                    Message {
                        role: Role::Assistant,
                        content: "There are several common reasons an async function might block:\n\n1. **CPU-bound work** - Heavy computation should use `spawn_blocking`\n2. **Sync I/O** - Using `std::fs` instead of `tokio::fs`\n3. **Missing `.await`** - Forgetting to await a future\n4. **Blocking mutex** - Using `std::sync::Mutex` instead of `tokio::sync::Mutex`\n\nCan you share the code that's causing the issue?".to_string(),
                        timestamp: now - Duration::days(1) - Duration::minutes(10),
                    },
                    Message {
                        role: Role::User,
                        content: "Here's the function:\n\n```rust\nasync fn process_data(data: Vec<u8>) {\n    let result = heavy_computation(&data); // This blocks!\n    save_result(result).await;\n}\n```".to_string(),
                        timestamp: now - Duration::days(1) - Duration::minutes(5),
                    },
                    Message {
                        role: Role::Assistant,
                        content: "The issue is `heavy_computation` is CPU-bound and runs synchronously. Wrap it with `spawn_blocking`:\n\n```rust\nasync fn process_data(data: Vec<u8>) {\n    let result = tokio::task::spawn_blocking(move || {\n        heavy_computation(&data)\n    }).await.unwrap();\n    save_result(result).await;\n}\n```".to_string(),
                        timestamp: now - Duration::days(1),
                    },
                ],
            },
            snippet: "Why is my async function blocking the event loop?".to_string(),
            match_spans: vec![(27, 35)], // "blocking"
        },
        // Codex conversation about testing
        SearchResult {
            session: Session {
                id: "jkl012".to_string(),
                source: SessionSource::CodexCli,
                project_name: "web-app".to_string(),
                cwd: "~/Projects/web-app".to_string(),
                timestamp: now - Duration::days(3),
                messages: vec![
                    Message {
                        role: Role::User,
                        content: "Write a test for the UserService.create method".to_string(),
                        timestamp: now - Duration::days(3) - Duration::minutes(2),
                    },
                    Message {
                        role: Role::Assistant,
                        content: "Here's a comprehensive test for `UserService.create`:\n\n```typescript\ndescribe('UserService.create', () => {\n  it('should create a new user with valid data', async () => {\n    const userData = { email: 'test@example.com', name: 'Test User' };\n    const user = await userService.create(userData);\n    \n    expect(user.id).toBeDefined();\n    expect(user.email).toBe(userData.email);\n    expect(user.name).toBe(userData.name);\n  });\n\n  it('should throw on duplicate email', async () => {\n    const userData = { email: 'existing@example.com', name: 'User' };\n    await expect(userService.create(userData)).rejects.toThrow('Email already exists');\n  });\n});\n```".to_string(),
                        timestamp: now - Duration::days(3),
                    },
                ],
            },
            snippet: "Write a test for the UserService.create method".to_string(),
            match_spans: vec![(8, 12)], // "test"
        },
        // Long conversation with multiple back-and-forths
        SearchResult {
            session: Session {
                id: "mno345".to_string(),
                source: SessionSource::ClaudeCode,
                project_name: "ml-pipeline".to_string(),
                cwd: "~/Research/ml-pipeline".to_string(),
                timestamp: now - Duration::days(7),
                messages: vec![
                    Message {
                        role: Role::User,
                        content: "I need help optimizing my neural network training loop".to_string(),
                        timestamp: now - Duration::days(7) - Duration::hours(1),
                    },
                    Message {
                        role: Role::Assistant,
                        content: "I'd be happy to help optimize your training loop. What framework are you using (PyTorch, TensorFlow, JAX)? And what specific performance issues are you seeing?".to_string(),
                        timestamp: now - Duration::days(7) - Duration::minutes(55),
                    },
                    Message {
                        role: Role::User,
                        content: "PyTorch. Training is slow even with GPU. Each epoch takes 10 minutes.".to_string(),
                        timestamp: now - Duration::days(7) - Duration::minutes(50),
                    },
                    Message {
                        role: Role::Assistant,
                        content: "Several things could help:\n\n1. **DataLoader optimization**:\n   - Increase `num_workers`\n   - Enable `pin_memory=True`\n   - Use `prefetch_factor`\n\n2. **Mixed precision training**:\n   ```python\n   scaler = torch.cuda.amp.GradScaler()\n   with torch.cuda.amp.autocast():\n       output = model(input)\n   ```\n\n3. **Gradient accumulation** for larger effective batch size\n\n4. **Compile the model** (PyTorch 2.0+):\n   ```python\n   model = torch.compile(model)\n   ```".to_string(),
                        timestamp: now - Duration::days(7),
                    },
                ],
            },
            snippet: "I need help optimizing my neural network training loop".to_string(),
            match_spans: vec![(17, 27)], // "optimizing"
        },
    ]
}
