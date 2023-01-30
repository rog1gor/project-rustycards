# RustyCards

## Authors
- Igor Kamiński (@rog1gor on GitHub)
- Jakub Owczarek (@obukaj on GitHub)

## Description
Rusty Cards is going to be a duel, card game with online gameplay.

## Features
- Lobby that enables connection beetween two players with GUI
- A duel with GUI:
  - Drawing cards
  - Gaining mana
  - Droping cards into the battlefield
  - Attacking with the cards on your battlefield

## Later Features
- Card collection
- Customizable decks

## Plan
In the first part we're going to implement the logic that stays behind the game. It will be possible  play the game with one deck, using command line (without GUI).

In the second part we're going to create a lobby and add GUI.

Later we will try to develop the project - we will add more cards, card collection and we will enable deck custommization.

## Libraries
- Serde (for serialization)
- bevy ECS (for game engine)
- nannou (graphics)

## First part of the project
- The client and the server (lobby like) is ready
- There are some delays with game logic
- To run the server you need to run cargo run in server directory
- To run the client you need to run cargo run in client directory
- To setup clients and server there are multiple command line messages that should help with the setup

## Second part of the project
- The client and the server are ready and the problems from the first part are solved
- The game logic is implemented - the game can be played remotely!
- Due to some problems I ended up writing these parts of project on my own, so I didn't have enaugh time to implement GUI other than the one in console (It is nice though)
- To run the server you need to run cargo run in server directory
- To run the client you need to run cargo run in client directory
- To setup clients and server there are multiple command line messages that should help with the setup
- There is a tutorial that will help you go through the game basics
