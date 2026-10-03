# Author Assistant Harness

This harness is for helping me write by researching and answering questions about my world and stories so far.

## Features

- [ ] Answer lore questions about any story in the multiverse
- [ ] Answer specific questions about character names, what they are holding, and their statuses
- [ ] Check for inconsistencies in stories
- [ ] Create a character card for any character

## Usage

`cargo run --release -- -p "create a character card for Xris."`

## ToDo

- [x] Initial Setup
  - [x] Get prompt from command line
  - [x] Send request to LLM
  - [x] Stream response to standard out
  - [x] Set system prompt
- [ ] Advertise tools
  - [x] Read file
  - [ ] List all files in directory
- [ ] Implement tools
  - [x] Read file
  - [ ] List all files in directory
- [ ] Multi-step
  - [ ] Set up memory
  - [ ] loop until done
- [ ] Set up judge
  - [ ] Review initial prompt and end result, determine if complete or keep trying
  
