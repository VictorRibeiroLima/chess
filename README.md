# Chess Game

This is a chess game monorepo. It contains the following packages:
- [Engine](#engine) - The chess engine
- [Chess AI](#chess-ai) - The chess AI
- [Server](#server) - The server
- [CLI Client](#cli-client) - The CLI client

## Engine

### TODO

- [x] Basic board representation
- [x] Pawn basic moves
- [x] Pawn capture moves
- [x] Pawn promotion
- [x] Pawn en passant
- [x] Pawn double move
- [x] Rook basic moves
- [x] Bishop basic moves
- [x] Queen basic moves
- [x] Knight basic moves
- [x] King basic moves
- [x] King castling
- [x] King check
- [x] King checkmate
- [ ] King stalemate
- [ ] Threefold repetition
- [ ] Fifty-move rule
- [ ] Dead position

## Chess AI 

### TODO

- [x] Basic move generation
- [ ] Basic evaluation function
- [ ] Minimax
- [ ] Alpha-beta pruning
- [ ] Iterative deepening
- [ ] Quiescence search

## Server

### Table of contents
- [Endpoints](#endpoints)
- [Room commands](#room-commands)
    - [Movement](#movement)
    - [Promote](#promote)
    - [Resign](#resign)
- [Responses](#responses)
    - [Error](#error)
    - [Success](#success)
### Endpoints
- `GET api/` - Health check
- `GET api/room` - Get all available rooms
- `GET ws/room/create` - Create a new room
- `GET ws/room/{id}` - Join an existing room

### Room commands
#### Movement
``` json
{
    "move": {
        "from": "a2",
        "to": "a4"
    }
}
```

### Promote
``` json
{
    "promote": {
        "piece": "queen"
    }
}
```

### Resign
``` json
{
    "resign": true
}
```

### Reset
``` json
{
    "reset": true
}
```

### Responses

#### Error
``` json
{
    "error": {
        "roomId": "1421a2b3-4c5d-6e7f-8g9h-0i1j2k3l4m5n",
        "clientId": "1421a2b3-4c5d-6e7f-8g9h-0i1j2k3l4m5n",
        "error": "Invalid move"
    }
}
```

#### Success
``` json
{
    "success": {
        "roomId": "1421a2b3-4c5d-6e7f-8g9h-0i1j2k3l4m5n",
        "clientId": "1421a2b3-4c5d-6e7f-8g9h-0i1j2k3l4m5n",
        "result": {
            "movement": {
                "valid": ["a2", "a3"],
                "promotion": null,
                "check": null,
            }
        }
    }
}
```

Available success results:
- `movement`
``` json
{
    "movement": {
        "valid": ["a2", "a3"],
        "promotion": null,
        "check": null,
    }
}

{
    "movement": {
        "capture": ["a7", "b8"],
        "promotion": "white",
        "check": "black"
    }
}

{
    "movement": {
        "enPassant": ["a5", "b6"],
        "promotion": null,
        "check": null,
    }
}

{
    "movement": {
        "castling": [["e1", "c1"], ["a1", "d1"]],
        "promotion": null,
        "check": null,
    }
}

{
    "movement": {
        "initialDoubleAdvance": ["a2", "a4"],
        "promotion": null,
        "check": null,
    }
}

```
- `promotion`
``` json
{
    "promotion": {
        "type": "queen",
        "position": "a8",
        "check": "black"
    }
}
```
- `winner`
``` json
{
    "winner": "white"
}
```
- `connect`
``` json
{
    "connect": {
        "roomId": "1421a2b3-4c5d-6e7f-8g9h-0i1j2k3l4m5n",
        "clientId": "1421a2b3-4c5d-6e7f-8g9h-0i1j2k3l4m5n",
        "color": "white"
    }
}
```
- `disconnect`
``` json
{
    "disconnect": {
        "roomId": "1421a2b3-4c5d-6e7f-8g9h-0i1j2k3l4m5n",
        "clientId": "1421a2b3-4c5d-6e7f-8g9h-0i1j2k3l4m5n"
    }
}
```


## CLI Client

### TODO

- [ ] Basic CLI client
- [ ] Initialization flags
- [ ] Console rendering
- [ ] Input handling
- [ ] Single player vs AI
- [ ] Local player vs player
- [ ] Online player vs player

## Web Client
Will be done in another repository





