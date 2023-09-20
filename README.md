# Chess Game

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
            "movement": ["a2", "a4"]
        }
    }
}
```

Available success results:
- `movement`
``` json
{
    "movement": ["a2", "a4"]
}
```
- `promotion`
``` json
{
    "promotion": ["a8", "queen"]
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





