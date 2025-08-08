# Complish Development Plan

Complish is a personal productivity CLI tool focused on task management, time tracking, and accomplishment reporting.
This document outlines the current state, planned features, and development roadmap.

## Current State

### Existing Entities

#### Projects

- CRUD operations with workflow management
- Fields: id, name, key, description, color, workflow_status, resolution, updates, timestamps
- Workflow states: Todo → Planned → InProgress → Done
- Resolutions: Completed, Canceled
- Support for tags and associated tasks
- Auto-generated 3-letter keys from project names

#### Tasks

- Comprehensive task management with rich metadata
- Fields: id, sequence_id, project_id, external_id, title, description, priority, workflow_status, estimation,
  resolution, notes, work_logs, due_at, timestamps
- Workflow states: Todo → InProgress → Blocked → Done
- Resolutions: Completed, Canceled, Delegated
- Work logging with start/stop tracking and source attribution
- Task relationships (blocks, depends_on, child_of, parent_of, etc.)
- Priority levels and estimations
- Notes system for task annotations

#### Tags

- Lightweight labeling system for both projects and tasks
- Many-to-many relationships via junction tables

#### Relationships

- Rich task-to-task relationships
- Types: blocks, child_of, cloned_by, cloned_from, depends_on, parent_of, related_to

### Current Database Schema

- SQLite-based storage with proper migrations
- Foreign key constraints and indexing
- JSON fields for complex data (notes, work_logs, updates)
- Auto-incrementing sequence IDs for tasks

## Planned New Entities

### Journal Entry

**Purpose**: Daily journaling for reflection and documentation

**Fields**:

- `id` (TEXT, PRIMARY KEY): Unique identifier
- `title` (TEXT): Optional entry title
- `content` (TEXT, NOT NULL): Journal entry content
- `date` (DATE, NOT NULL): The date this entry represents
- `created_at` (TIMESTAMP): When the entry was created
- `updated_at` (TIMESTAMP): When the entry was last modified

**Features**:

- Multiple entries per day allowed
- Full-text search capabilities
- Integration with accomplishment reports

### Git Commit Log

**Purpose**: Automatic capture of development work from local git repositories

**Fields**:

- `id` (TEXT, PRIMARY KEY): Unique identifier
- `repository_path` (TEXT, NOT NULL): Local path to the git repository
- `commit_hash` (TEXT, NOT NULL): Git commit SHA
- `author_name` (TEXT, NOT NULL): Commit author name
- `author_email` (TEXT, NOT NULL): Commit author email
- `message` (TEXT, NOT NULL): Commit message
- `files_changed` (TEXT): JSON array of changed files
- `insertions` (INTEGER): Lines added
- `deletions` (INTEGER): Lines deleted
- `committed_at` (TIMESTAMP, NOT NULL): When the commit was made
- `imported_at` (TIMESTAMP, NOT NULL): When this record was imported

**Features**:

- Automatic repository scanning and import
- Configurable repository inclusion/exclusion
- Deduplication to prevent duplicate imports
- Integration with accomplishment reports

### Daily Sprint

**Purpose**: Daily planning and goal setting with carry-over task management

**Fields**:

- `id` (TEXT, PRIMARY KEY): Unique identifier
- `date` (DATE, NOT NULL): The sprint date
- `status` (TEXT, NOT NULL): active, completed, abandoned
- `planned_task_count` (INTEGER, DEFAULT 10): Target number of tasks
- `created_at` (TIMESTAMP): When the sprint was created
- `completed_at` (TIMESTAMP): When the sprint was completed
- `updated_at` (TIMESTAMP): When the sprint was last modified

**Features**:

- 24-hour duration (midnight to midnight)
- Configurable target task count (default: 10)
- Automatic carryover task handling
- Integration with task workflow

### Sprint Task Association

**Purpose**: Many-to-many relationship between sprints and tasks

**Fields**:

- `sprint_id` (TEXT, NOT NULL): Reference to sprint
- `task_id` (TEXT, NOT NULL): Reference to task
- `added_from_carryover` (BOOLEAN, DEFAULT FALSE): Whether this task was carried over from previous sprint
- `created_at` (TIMESTAMP): When the association was created

## Core Features

### Task and Project Management

- 🔄 Create, update, and manage tasks and projects
- 🔄 Move tasks through defined workflows
- 🔄 Associate tasks with projects and other tasks
- 🔄 Start, pause, and stop work on tasks with time tracking
- 🔄 Complete or cancel tasks and projects
- 🔄 Enhanced bulk operations for task management

### Git Integration

- 🆕 Automatic local git repository discovery and scanning
- 🆕 Commit log import with metadata extraction
- 🆕 Configurable repository inclusion/exclusion patterns
- 🆕 Deduplication and incremental import strategies

### Journaling System

- 🆕 Daily journal entry creation and management
- 🆕 Multiple entries per day support
- 🆕 Full-text search across journal entries
- 🆕 Rich text formatting support

### Daily Sprint System

- 🆕 Interactive sprint planning workflow
- 🆕 Configurable daily task targets
- 🆕 Smart carryover task management
- 🆕 Sprint completion tracking and analytics

### Accomplishment Reporting

- 🆕 Comprehensive accomplishment reports for date ranges
- 🆕 Integration of completed tasks, created tasks, git history, and journal entries
- 🆕 LLM-ready output formatting for employment review generation
- 🆕 Multiple output formats (markdown, JSON, plain text)

## CLI Commands

### Core Commands

- `complish sprint` (default): Start or continue daily sprint planning
- `complish task create <title>`: Create a new task
- `complish task start <id>`: Start working on a task
- `complish task stop <id>`: Stop working on a task
- `complish task complete <id>`: Mark a task as complete
- `complish project create <name>`: Create a new project

### Sprint Management

- `complish sprint start`: Begin a new daily sprint
- `complish sprint show`: Show current sprint status
- `complish sprint complete`: Complete the current sprint

### Journaling

- `complish journal add [title]`: Create a new journal entry
- `complish journal list [--date DATE]`: List journal entries
- `complish journal search <query>`: Search journal entries

### Git Integration

- `complish git scan [path]`: Scan and import git repositories
- `complish git status`: Show git import status
- `complish git config`: Configure git scanning settings

### Reporting

- `complish report accomplishments <start-date> <end-date>`: Generate accomplishment report
- `complish report sprint <date>`: Generate sprint summary report

## Daily Sprint Workflow

### Default Command Behavior

When users run `complish` with no arguments, the system will:

1. **Check for Active Sprint**: Look for an active sprint for the current date
2. **Sprint Planning Mode**: If no active sprint exists, initiate the sprint planning workflow
3. **Help Mode**: If an active sprint exists, display help text and current status

### Sprint Planning Workflow (when no active sprint exists)

1. **Create New Sprint**: Create a new sprint for today
2. **Handle Carryover Tasks**:

    - Identify incomplete tasks from the previous sprint
    - Present each carryover task to the user with options:
      - Mark as complete (if actually finished)
      - Include in today's sprint
      - Move back to backlog (deprioritize)

3. **Sprint Planning**:

    - Show current sprint task count
    - Prompt for additional tasks to reach target (default: 10 total)
    - Allow task selection from backlog or creation of new tasks

4. **Sprint Activation**: Mark sprint as active and ready for execution

### Sprint Task Management

- Tasks in active sprints are prioritized in task lists
- Work logging automatically associates with the current sprint
- Sprint completion triggers summary generation

## Accomplishment Reporting System

### Report Components

The accomplishment report aggregates data from multiple sources:

1. **Task Activity**:

    - Tasks completed within the date range
    - Tasks created within the date range
    - Work log summaries and time tracking

2. **Git Activity**:

    - Commits made within the date range
    - Repository activity summaries
    - Code change statistics (insertions/deletions)

3. **Journal Entries**:

    - All journal entries within the date range
    - Searchable and categorizable content

4. **Sprint Summaries**:

    - Sprint completion rates
    - Sprint goal achievement
    - Carryover task patterns

### Report Generation

Reports are generated in multiple formats optimized for different use cases:

- **LLM Format**: Structured markdown optimized for feeding to Claude/GPT for employment review generation
- **Executive Summary**: High-level overview for management reporting
- **Detailed Timeline**: Chronological view of all activities
- **Analytics**: Quantitative metrics and productivity insights

### Integration with LLMs

The generated reports are specifically formatted to work well with Large Language Models:

- Clear section delineation and hierarchical structure
- Quantitative data presented with context
- Qualitative entries (journal, commit messages) preserved for analysis
- Standardized formatting for consistent LLM interpretation

## Technical Implementation Notes

### Database Considerations

- Maintain backward compatibility with existing schema
- Add new tables with appropriate foreign key relationships
- Consider partitioning strategies for high-volume data (git commits)
- Implement proper indexing for date-based queries

### Performance Optimization

- Git scanning should be incremental and configurable
- Journal search requires full-text indexing
- Report generation should cache intermediate results
- Consider async processing for long-running operations

### Configuration Management

- Repository scanning patterns and exclusions
- Daily sprint target configurations
- Report formatting templates
- Git author mapping and normalization

### Error Handling and Resilience

- Graceful handling of repository access issues
- Data validation for all new entities
- Rollback capabilities for failed operations
- Comprehensive logging for debugging

## Development Phases

### Phase 1: Core Entity Implementation

- [ ] Implement Task entity and CLI commands
- [ ] Implement Project entity and CLI commands
- [ ] Implement Journal Entry entity and CLI commands
- [ ] Implement Git Commit Log entity and scanning logic
- [ ] Implement Daily Sprint entity and associations
- [ ] Add database migrations for all new tables

### Phase 2: Sprint Workflow

- [ ] Implement default sprint command behavior
- [ ] Build carryover task management interface
- [ ] Create sprint planning interactive prompts
- [ ] Add sprint completion and summary features

### Phase 3: Git Integration

- [ ] Build repository discovery and scanning engine
- [ ] Implement incremental import strategies
- [ ] Add configuration system for repository management
- [ ] Create git activity analysis and reporting

### Phase 4: Accomplishment Reporting

- [ ] Build report generation engine
- [ ] Implement multiple output formats
- [ ] Create LLM-optimized formatting
- [ ] Add report caching and optimization

### Phase 5: Natural Language Interface

- [ ] Implement natural language command parsing
- [ ] Add support for date/time expressions (tomorrow, next week, etc.)
- [ ] Create intelligent parameter extraction from natural language
- [ ] Add command suggestion and autocorrection

### Phase 6: External Integrations

- [ ] Design plugin architecture for external systems
- [ ] Implement Jira integration (issue import/sync)
- [ ] Implement Linear integration (issue import/sync)
- [ ] Implement GitHub/GitLab integration (issue/PR import)
- [ ] Implement Shortcut integration
- [ ] Add bidirectional sync capabilities

### Phase 7: Cloud Sync and Collaboration

- [ ] Design cloud sync architecture
- [ ] Implement encrypted data synchronization
- [ ] Add multi-device support
- [ ] Create conflict resolution strategies
- [ ] Add team collaboration features

### Phase 8: Local LLM Integration

- [ ] Add LLM API integration (OpenAI, Anthropic, local models)
- [ ] Implement automated summary generation
- [ ] Add intelligent task categorization and prioritization
- [ ] Create contextual assistance and suggestions
- [ ] Add natural language query interface

### Phase 9: Polish and Enhancement

- [ ] Add comprehensive test coverage
- [ ] Implement advanced search and filtering
- [ ] Create data export/import capabilities
- [ ] Add performance monitoring and optimization

## Configuration

The system should support configuration at multiple levels:

- Global user configuration
- Per-project configuration
- Environment variable overrides

### Key Configuration Areas

- Git repository inclusion/exclusion patterns
- Daily sprint target task counts
- Report formatting preferences
- Journal entry templates
- Time tracking preferences

## Future Vision

### External System Integrations

Complish will integrate with popular project management and development tools:

**Issue Tracking Systems**:

- Jira (Atlassian)
- Linear
- Shortcut (formerly Clubhouse)
- GitHub Issues
- GitLab Issues
- Azure DevOps Work Items

**Integration Capabilities**:

- Import tasks/issues as Complish tasks with external_id linking
- Bidirectional sync of task status changes
- Automatic work log synchronization
- Comment and update synchronization
- Sprint/milestone mapping

**Integration Architecture**:

- Plugin-based system for extensibility
- OAuth/API token authentication
- Configurable sync intervals and conflict resolution
- Selective sync with filtering rules

### Cloud Synchronization

**Multi-Device Support**:

- Encrypted cloud storage of Complish data
- Real-time synchronization across devices
- Offline-first design with conflict resolution
- Team collaboration and shared workspaces

**Cloud Features**:

- Automatic backup and restore
- Cross-platform data access
- Team sprint coordination
- Shared accomplishment reporting

### Local LLM Integration

**AI-Powered Features**:

- Automated accomplishment summary generation using personal API tokens
- Intelligent task categorization and tagging
- Natural language task creation and querying
- Smart sprint planning suggestions based on historical data
- Context-aware productivity insights

**Supported LLM Providers**:

- OpenAI (GPT-4, GPT-3.5)
- Anthropic (Claude)
- Local models via Ollama
- Custom API endpoints

**Privacy-First Design**:

- All LLM processing with user-provided API tokens
- No data sent to third parties without explicit consent
- Local model support for maximum privacy
- Configurable data retention policies

- Daily sprint completion rates
- Task completion velocity
- Git activity correlation with task work
- Report generation usage and feedback
- Overall user engagement with the system

---

> [!NOTE]
> This document represents the current planning state and will be updated as development progresses and requirements
> evolve.
