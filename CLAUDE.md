# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a photobooth application project. It is in early stages with the speckit feature specification workflow already configured.

## Available Commands

This project uses the speckit workflow for feature-driven development:

- `/speckit.specify` - Create a feature specification from a natural language description
- `/speckit.plan` - Generate a technical implementation plan from a spec
- `/speckit.tasks` - Generate actionable tasks from a plan
- `/speckit.implement` - Execute the implementation plan
- `/speckit.checklist` - Generate a checklist for the current feature
- `/speckit.clarify` - Identify underspecified areas in the current feature spec
- `/speckit.analyze` - Cross-artifact consistency analysis (spec.md, plan.md, tasks.md)
- `/speckit.constitution` - Create or update the project constitution
- `/speckit.taskstoissues` - Convert tasks to GitHub issues

## Architecture

The speckit workflow uses the following structure:
- `.specify/templates/` - Templates for specs, plans, tasks, and checklists
- `.specify/memory/constitution.md` - Project constitution (core principles and constraints)
- `.specify/scripts/bash/` - Helper scripts for feature creation and branch management
- `.claude/commands/` - Claude Code command definitions

Features are developed in feature branches with specs stored in `specs/[number]-[feature-name]/` directories.

## Development Workflow

1. Use `/speckit.specify <feature description>` to create a new feature specification
2. Use `/speckit.plan` to create a technical plan from the spec
3. Use `/speckit.tasks` to generate implementation tasks
4. Use `/speckit.implement` to execute the tasks
