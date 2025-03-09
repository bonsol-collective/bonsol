# Story Template

<version>1.0.0</version>

## Requirements

- Follow standardized story structure
- Include all required sections - some will be empty to start with

## Story Structure

### Required Sections

#### 1. Header

Epic-{N}: {Epic Title}
Story-{M}: {Story Title}

#### 2. Story Description

**As a** {role}
**I want** {action}
**so that** {benefit}

#### 3. Status

- Draft
- In Progress
- Complete
- Cancelled

#### 4. Context

- Background information
- Current state
- Story justification
- Technical context
- Business drivers
- Relevant history from previous stories

#### 5. Estimation

Story Points: {Story Points (1 SP = 1 day of Human Development = 10 minutes of AI development)}

#### 6. Tasks

##### Task Groups

1. - [ ] {Major Task Group 1}
   1. - [ ] {Test Subtasks (as needed)}
   2. - [ ] {Subtask}

Note:

- Use - [x] for completed items
- Use ~~skipped/cancelled items~~
- Follow TDD - Start with Writing Tests Subtask

### 7. Additional Sections

#### Constraints

- List any technical or business constraints

#### Data Models / Schema

- Database schemas
- API request/response models
- Interfaces/types

#### Structure

- Module organization
- File structure plan

#### Diagrams

- Mermaid diagrams as needed

#### Dev Notes

- Implementation commentary
- Important considerations
- Technical decisions

#### Chat Command Log

- Commands from User
- Agent Question and Users Response

## Examples

<example>
# Epic-1: Basic Chess Game
# Story-2: Implement Chessboard UI

## Story

**As a** chess player
**I want** to see a clear and interactive chessboard
**so that** I can play chess in the web browser

## Status

In Progress

## Context

Part of Epic-1 which implements the core 2-player chess game. This story focuses on the visual and interactive aspects of the chessboard. The project setup (Story-1) is complete, providing the foundation for UI implementation.

## Estimation

Story Points: 2

## Tasks

1. - [x] Create Chessboard Grid
   1. - [x] Implement 8x8 board layout
   2. - [x] Add square coloring
   3. - [x] Write grid tests
2. - [ ] Add Chess Pieces
   1. - [ ] Create piece components
   2. - [ ] Add piece images
   3. - [ ] Write piece tests
3. - [ ] Implement Basic Interaction
   1. - [ ] Add click handlers
   2. - [ ] Highlight selected square
   3. - [ ] Write interaction tests

## Dev Notes

- Using React for UI components
- Chess piece SVGs from open-source library
- Implementing responsive design for various screen sizes

## Chat Command Log

- BMad: Let's implement the chessboard UI
- AiAgent: Reviewing the design requirements. Should we use SVG or PNG for pieces?
  - BMad: Let's use SVG for better scaling
- AiAgent: Grid implementation complete, proceeding with piece placement
  </example>

<example type="invalid">
Chess UI Story

todo:

- make board
- add pieces
- make it work
  </example>
