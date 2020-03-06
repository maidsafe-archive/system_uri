# Contributing to the SAFE Network

:tada: Thank you for your interest in contributing to the SAFE Network! :tada:

This document is a set of guidelines for contributing to the SAFE Network. These are guidelines, not rules. This guide is designed to make it easy for you to get involved.

Notice something amiss? Have an idea for a new feature? Feel free to create an issue in this GitHub repository about anything that you feel could be fixed or improved. Examples include:
- Bugs, crashes
- Enhancement ideas
- Unclear documentation
- Lack of tutorials and hello world examples
- ... and more

See our [Issues and Feature Requests](#issues-and-feature-requests) section below for further information on creating new issues.

Of course, after submitting an issue you are free to assign it to yourself and tackle the problem, or pick up any of the other outstanding issues yet to be actioned - see the [Development](#development) section below for more information.

Further support is available [here](#support).

This project adheres to the [Contributor Covenant](https://www.contributor-covenant.org/). By participating, we sincerely hope that you honour this code.

## What we're working on

The best way to follow our progress is to read the [MaidSafe Dev Updates](https://safenetforum.org/c/development/updates), which are published every week (on Thursdays) on the [SAFE Network Forum](https://safenetforum.org/).

See our [Development Roadmap](https://safenetwork.tech/roadmap/) for more information on our near term development focus and longer term plans.

## Issues and Feature Requests

We are in the process of adding issue templates to each MaidSafe repository, with guidance and required information specific to that repository detailed within. Opening an issue in each repository will auto-populate your issue with this template.

As per the issue template, issues should clearly lay out the problem, platform(s) experienced on, as well as steps to reproduce the issue. This aids in fixing the issue and validating that the issue has indeed been fixed if the reproduction steps are followed.

Each issue is labelled by the team depending on its type, typically the standard labels we use are:
* `bug`: the issue is a bug in the product
* `feature`: the issue is a new and non-existent feature to be implemented in the product
* `enhancement`: the issue is an enhancement to either an existing feature in the product or to the infrastructure around the development process of the product
* `blocked`: the issue cannot be resolved as it depends on a fix in any of its dependencies
* `good first issue`: an issue considered more accessible for any developer who would like to start contributing
* `help wanted`: an issue considered lower priority for the MaidSafe team, but one that would appear to be suitable for an outside developer who would like to contribute

These labels are meant as a soft guide, if you want to work on an issue which doesn't have a `good first issue` or `help wanted` label, by all means fill your boots!

## Development

At MaidSafe, we follow a common development process. We use [Git](https://git-scm.com/) as our [version control system](https://en.wikipedia.org/wiki/Version_control). We develop new features in separate Git branches, raise [pull requests](https://help.github.com/en/articles/about-pull-requests), put them under peer review, and merge them only after they pass QA checks and [continuous integration](https://en.wikipedia.org/wiki/Continuous_integration) (CI). We do not commit directly to the `master` branch.

For useful resources, please see:

- [Git basics](https://git-scm.com/book/en/v1/Getting-Started-Git-Basics) for Git beginners
- [Git best practices](https://sethrobertson.github.io/GitBestPractices/)

We ask that if you are working on a particular issue, you ensure that the issue is logged in the GitHub repository and you assign that issue to yourself to prevent duplication of work.

### Code Style

In our [Rust Programming Language](https://www.rust-lang.org/) repositories we follow the company-wide code style guide that you can find in the [the Rust Style document](https://github.com/maidsafe/QA/blob/master/Documentation/Rust%20Style.md). You should install `rustfmt` and `clippy` and run them before each of your Git commits.

For our non-Rust repositories we follow the standard lint suggestions, pre-linting before commit. We encourage our contributors to use a sensible naming convention, split their files up accordingly, and include accompanying tests.

### Commits

We use the [Conventional Commit](https://www.conventionalcommits.org/en/v1.0.0-beta.3/) message style, usually including a scope. You can have a look at the commit history within each repository to see examples of our commits.

All code should be pre-linted before commit. The use of pre-commit Git hooks is highly recommended to catch formatting and linting errors early.

### Pull Requests

If you are a newbie to pull requests (PRs), click [here](https://github.com/firstcontributions/first-contributions) for an easy-to-follow guide (with pictures!).

We follow the standard procedure for submitting PRs. Please refer to the [official GitHub documentation](https://help.github.com/articles/creating-a-pull-request/) if you are unfamiliar with the procedure. If you still need help, we are more than happy to guide you along!

We are in the process of adding pull request templates to each MaidSafe repository, with guidance specific to that repository detailed within. Opening a PR in each repository will auto-populate your PR with this template. PRs should clearly reference an issue to be tracked on the project board. A PR that implements/fixes an issue is linked using one of the [GitHub keywords](https://help.github.com/articles/closing-issues-using-keywords) - note that these types of PRs will not be added themselves to a project board (to avoid redundancy with the linked issue). However, PRs which were submitted spontaneously and not linked to any existing issue will be added to the project board so they can be tracked, and should go through the same process as any other task/issue.

Pull requests should strive to tackle one issue/feature, and code should be pre-linted before commit.

Ideally, a multi-commit PR should be a sequence of commits "telling a story", going in atomic and easily reviewable steps from the initial to the final state.

Each PR should be rebased on the latest upstream commit; avoid merging from the upstream branch into the feature branch/PR. This means that a PR will probably see one or more force-pushes to keep up to date with changes in the upstream branch.

Fixes to review comments should preferably be pushed as additional commits to make it easier for the reviewer to see the changes. As a final step once the reviewer is happy the author should consider squashing these fixes with the relevant commit.

Smaller PRs can have their commits squashed together and fast-forward merged, while larger PRs should probably have the chain of commits left intact and fast-forward merged into the upstream branch.

Where appropriate, commits should always contain tests for the code in question.

#### Running tests (CI script)

Submitted PRs are expected to pass continuous integration (CI), which, among other things, runs a test suite on your PR to make sure that your code has not regressed the code base.

#### Code Review

Your PR will be automatically assigned to the team member(s) specified in the `codeowners` file, who may either review the PR himself/herself or assign it to another team member. More often than not, a code submission will be met with review comments and changes requested. It's nothing personal, but nobody's perfect; we leave each other review comments all the time.

Fixes to review comments should preferably be pushed as additional commits to make it easier for the reviewer to see the changes. As a final step once the reviewer is happy the author should consider squashing these fixes with the relevant commit.

### Project board

GitHub project boards are used by the maintainers of the majority of our repositories to keep track of progress and organise development priorities.

There may be one or more active project boards for a repository. Typically, one main project is used to manage all tasks corresponding to the main development stream (normally the `master` branch), while a separate project would be used to manage each proof of concept or milestone, and each of them will track a dedicated development branch.

New features which involve a large number of changes may be developed in a dedicated feature branch, but would normally be tracked on the same main project board as the main development branch (normally `master` branch), re-basing it with the main branch regularly and fully testing the feature on its own branch before it is fully approved and merged into the main branch.

The main project boards typically contain the following Kanban columns to track the status of each development task:

* **To do**: new issues which need to be reviewed and evaluated to decide their priority, add labels, clarify, etc.
* **In Progress**: the task is assigned to a person and it is in progress
* **Needs Review**: the task is considered complete by the assigned developer and so has been sent for peer review
* **Reviewer approved**: the task has been approved by the reviewer(s) and is considered ready to be merged
* **Done**: the PR associated with the task was merged (or the task was completed by any other means)

The project board columns would typically include automation to move the issues between columns upon set actions, for example, if a PR was created which indicated in its description that it resolved a particular issue on the project board (using [GitHub keywords](https://help.github.com/articles/closing-issues-using-keywords)) then that issue would automatically be moved to the `Done` column on the board on PR merge.

## Releases and Changelog

The release process is triggered by the maintainers of each repository, with versioning increments according to the [Semantic Versioning specification](https://semver.org/). Releases are typically generated through our CI setup, which releases upon a trigger commit title (e.g. `Version change...`), or through specific programming language release tools such as `cargo release` or `yarn bump`.

Typically, we only update/regenerate the [CHANGELOG file](CHANGELOG.md) with the latest changes on a new version release, where all changes since the last release are then added to the changelog file.

If a repository is for a library, or perhaps multiple libraries, then often no release artefact is produced. A tag would always be added to the repository on each release though, these tags can be viewed in the `/releases` page of each repository. Repositories which do produce artefacts, such as `.AppImage`, `.dmg` or `.exe` files, will have the release files available in the repository's `/release` page, or instructions there on how to obtain it.

## Support

Contributors and users can get support through the following official channels:
* GitHub issues: Log an issue in the repository where you require support.
* [SAFE Network Forum](https://safenetforum.org/): Join our community forum, say hi, and discuss your support needs and questions with likeminded people.
* [SAFE Dev Forum](https://forum.safedev.org/): Need to get technical with other developers? Join our developer forum and post your thoughts and questions.
* [SAFE Network chat rooms](https://safenetforum.org/t/safe-network-chat-rooms/26070): The General chat room is a good place to ask for help. There is also a Development chat room for more technical discussion.
