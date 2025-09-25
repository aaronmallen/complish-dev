# Complish

> [!NOTE]
> This project has moved to [https://github.com/aaronmallen/complish](https://github.com/aaronmallen/complish)

[![GitHub Actions Workflow Status][build-badge]][build-status]
[![Codacy Quality][codacy-quality-badge]][codacy]
[![Codacy Coverage][codacy-coverage-badge]][codacy-coverage]
[![GitHub Sponsors][sponsor-badge]][sponsor]
[![GitHub License][license-badge]][license]

> [!CAUTION]
> This project is currently in early development. Rebases and force pushes will occur.

## Personal Productivity with Accomplishment Recall

Complish is a personal productivity tool designed around a core principle: **you should be able to recall what you
accomplished during any time period**.

## The Problem

Most task managers focus on what you need to do next, but they're terrible at helping you remember what you've already
done. When performance review time comes, or a client asks for a project update, or you just want to reflect on your
productivity, you're left scrambling through scattered records trying to piece together your accomplishments.

## The Solution

Complish captures your accomplishments from multiple sources and makes them searchable and reportable:

- **Task Management**: Create, organize, and complete tasks with detailed work logging
- **Automatic Git Integration**: Capture commits from your local repositories as accomplishment records
- **Daily Journaling**: Document context, decisions, and non-task work
- **Sprint Planning**: Structure your work in manageable daily chunks
- **Comprehensive Reporting**: Generate accomplishment reports for any date range

## Core Value Proposition

Ask Complish "What did I accomplish between March 1st and March 15th?" and get a comprehensive report including:

- Tasks completed with time invested
- Code commits with change summaries
- Journal entries providing context
- Sprint completion rates and patterns

The generated reports are specifically formatted to work seamlessly with Large Language Models like Claude or ChatGPT.
Simply feed the report to your preferred LLM with a prompt like "Please generate a professional performance review
summary based on this accomplishment data" and get a polished, well-written document ready for submission.

Perfect for performance reviews, client updates, billing documentation, or personal reflection.

## Features

### Task Management

- Create, organize, and prioritize tasks with rich metadata
- Track time spent working on tasks with start/stop logging
- Establish relationships between tasks (dependencies, blockers, parent/child)
- Organize tasks into projects with workflow management
- Tag tasks and projects for flexible categorization

### Daily Sprint Planning

- Structure your work into focused daily sprints
- Set daily task targets and track completion rates
- Handle carryover tasks from previous days intelligently
- Review sprint performance over time

### Automatic Git Integration

- Discover and scan local git repositories automatically
- Import commit history as accomplishment records
- Track code changes, file modifications, and development activity
- Correlate coding work with task completion

### Personal Journaling

- Create daily journal entries for context and reflection
- Document decisions, meetings, and non-task work
- Full-text search across all journal content
- Integrate journal insights into accomplishment reports

### Comprehensive Reporting

- Generate detailed accomplishment reports for any date range
- Include task completions, git activity, and journal entries
- Output formatted specifically for LLM processing
- Export data in multiple formats (Markdown, JSON, plain text)

## Future Vision

While the initial release(s) focus on generating LLM-ready reports, future versions may integrate directly with AI
services to provide automated summary generation, intelligent task categorization, and contextual productivity insights
while keeping your data under your control.

## Design Philosophy

Every feature in Complish serves the accomplishment recall mission. This isn't a task manager with reporting added on;
it's an accomplishment capture system that happens to manage tasks as one of its data sources.

## Available Interfaces

Complish will work across multiple interfaces to fit different workflows and preferences:

- **Command Line Interface**: For developers and power users who prefer terminal-based workflows
- **Desktop Applications**: Native GUI apps for visual task management and intuitive interaction
- **Mobile Apps**: iOS and Android companions for on-the-go task management
- **Cloud Sync**: Encrypted synchronization across all your devices
- **Web Interface**: Browser-based access for teams and cross-platform compatibility

Choose the interface that best fits your workflow, or use multiple interfaces as your needs change throughout the day.

## Contributing

We welcome contributions! Please see our [contribution guidelines][contributing-md] for details on how to get started,
our development process, and coding standards.

Please note that this project is released with a Contributor Code of Conduct. By participating in this project you agree
to abide by its terms. See our [code of conduct][code-of-conduct-md] for details.

## License

This project is licensed under the [MIT License][license].

[build-badge]:
https://img.shields.io/github/actions/workflow/status/aaronmallen/complish-dev/build.yml?branch=main&style=for-the-badge&logo=githubactions&logoColor=white
[build-status]: https://github.com/aaronmallen/complish-dev/actions/workflows/build.yml
[code-of-conduct-md]: https://github.com/aaronmallen/complish-dev/blob/main/docs/CODE_OF_CONDUCT.md
[codacy]: https://app.codacy.com/gh/aaronmallen/complish-dev
[codacy-coverage]: https://app.codacy.com/gh/aaronmallen/complish-dev/coverage
[codacy-coverage-badge]: https://img.shields.io/codacy/coverage/c159dfde33fd47e6b755718c7dc24c78?style=for-the-badge&logo=codacy
[codacy-quality-badge]: https://img.shields.io/codacy/grade/c159dfde33fd47e6b755718c7dc24c78?style=for-the-badge&logo=codacy
[contributing-md]: https://github.com/aaronmallen/complish-dev/blob/main/docs/CONTRIBUTING.md
[license]: https://github.com/aaronmallen/complish-dev/blob/main/LICENSE
[license-badge]: https://img.shields.io/github/license/aaronmallen/complish-dev?style=for-the-badge&color=blue
[sponsor]: https://github.com/sponsors/aaronmallen
[sponsor-badge]:
https://img.shields.io/github/sponsors/aaronmallen?style=for-the-badge&logo=githubsponsors&logoColor=white
