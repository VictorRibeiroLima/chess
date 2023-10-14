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
- `timer`
``` json
{
   "timer":{
      "clientId":"6cc64dcf-8db9-4615-8405-55dc86360316",
      "roomId":"9128de01-9e6d-40a4-9628-3791fd622a8d",
      "time":152235,
      "color":"white"
   }
}
```

- `promotion`
``` json
{
    "promotion": {
        "piece": "queen",
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
    "connect":{
      "roomId":"7963de36-5b20-4932-b774-0f88ef1759d0",
      "clientId":"e3445dda-a7ca-480c-80b0-8e3bf8fd7f00",
      "enemyId":"196bfcc3-9613-4c9c-bd1b-da095ec0e75f",
      "conType":"selfClient",
      "color":"white",
      "check":null,
      "promotion":null,
      "pieces":[
         [
            {
               "type":"rook",
               "color":"white",
               "moved":false
            },
            {
               "type":"knight",
               "color":"white",
               "moved":false
            },
            {
               "type":"bishop",
               "color":"white",
               "moved":false
            },
            {
               "type":"queen",
               "color":"white",
               "moved":false
            },
            {
               "type":"king",
               "color":"white",
               "moved":false
            },
            {
               "type":"bishop",
               "color":"white",
               "moved":false
            },
            {
               "type":"knight",
               "color":"white",
               "moved":false
            },
            {
               "type":"rook",
               "color":"white",
               "moved":false
            }
         ],
         [
            {
               "type":"pawn",
               "color":"white",
               "moved":false
            },
            {
               "type":"pawn",
               "color":"white",
               "moved":false
            },
            {
               "type":"pawn",
               "color":"white",
               "moved":false
            },
            null,
            {
               "type":"pawn",
               "color":"white",
               "moved":false
            },
            {
               "type":"pawn",
               "color":"white",
               "moved":false
            },
            {
               "type":"pawn",
               "color":"white",
               "moved":false
            },
            null
         ],
         [
            null,
            null,
            null,
            null,
            null,
            null,
            null,
            null
         ],
         [
            null,
            null,
            null,
            {
               "type":"knight",
               "color":"black",
               "moved":true
            },
            null,
            null,
            null,
            null
         ],
         [
            null,
            null,
            null,
            null,
            null,
            {
               "type":"bishop",
               "color":"black",
               "moved":true
            },
            null,
            null
         ],
         [
            null,
            {
               "type":"queen",
               "color":"black",
               "moved":true
            },
            null,
            null,
            null,
            null,
            null,
            null
         ],
         [
            {
               "type":"pawn",
               "color":"black",
               "moved":false
            },
            {
               "type":"pawn",
               "color":"black",
               "moved":false
            },
            null,
            null,
            {
               "type":"bishop",
               "color":"black",
               "moved":true
            },
            {
               "type":"pawn",
               "color":"black",
               "moved":false
            },
            null,
            {
               "type":"pawn",
               "color":"black",
               "moved":false
            }
         ],
         [
            null,
            null,
            {
               "type":"king",
               "color":"black",
               "moved":true
            },
            {
               "type":"rook",
               "color":"black",
               "moved":true
            },
            null,
            null,
            {
               "type":"knight",
               "color":"black",
               "moved":false
            },
            {
               "type":"queen",
               "color":"white",
               "moved":false
            }
         ]
      ],
      "moves":[
         {
            "turnNumber":1,
            "piece":{
               "type":"pawn",
               "color":"white",
               "moved":true
            },
            "clientId":"fc98bd7c-9595-4e12-a4f9-6fb0e453e3c3",
            "movement":{
               "initialDoubleAdvance":[
                  "d2",
                  "d4"
               ]
            }
         },
         {
            "turnNumber":1,
            "piece":{
               "type":"pawn",
               "color":"black",
               "moved":true
            },
            "clientId":"196bfcc3-9613-4c9c-bd1b-da095ec0e75f",
            "movement":{
               "initialDoubleAdvance":[
                  "c7",
                  "c5"
               ]
            }
         },
         {
            "turnNumber":2,
            "piece":{
               "type":"pawn",
               "color":"white",
               "moved":true
            },
            "clientId":"fc98bd7c-9595-4e12-a4f9-6fb0e453e3c3",
            "movement":{
               "capture":[
                  "d4",
                  "c5"
               ]
            }
         },
         {
            "turnNumber":2,
            "piece":{
               "type":"pawn",
               "color":"black",
               "moved":true
            },
            "clientId":"196bfcc3-9613-4c9c-bd1b-da095ec0e75f",
            "movement":{
               "valid":[
                  "d7",
                  "d6"
               ]
            }
         },
         {
            "turnNumber":3,
            "piece":{
               "type":"pawn",
               "color":"white",
               "moved":true
            },
            "clientId":"fc98bd7c-9595-4e12-a4f9-6fb0e453e3c3",
            "movement":{
               "capture":[
                  "c5",
                  "d6"
               ]
            }
         },
         {
            "turnNumber":3,
            "piece":{
               "type":"queen",
               "color":"black",
               "moved":true
            },
            "clientId":"196bfcc3-9613-4c9c-bd1b-da095ec0e75f",
            "movement":{
               "valid":[
                  "d8",
                  "b6"
               ]
            }
         },
         {
            "turnNumber":4,
            "piece":{
               "type":"pawn",
               "color":"white",
               "moved":true
            },
            "clientId":"fc98bd7c-9595-4e12-a4f9-6fb0e453e3c3",
            "movement":{
               "capture":[
                  "d6",
                  "e7"
               ]
            }
         },
         {
            "turnNumber":4,
            "piece":{
               "type":"bishop",
               "color":"black",
               "moved":true
            },
            "clientId":"196bfcc3-9613-4c9c-bd1b-da095ec0e75f",
            "movement":{
               "valid":[
                  "c8",
                  "f5"
               ]
            }
         },
         {
            "turnNumber":5,
            "piece":{
               "type":"pawn",
               "color":"white",
               "moved":true
            },
            "clientId":"fc98bd7c-9595-4e12-a4f9-6fb0e453e3c3",
            "movement":{
               "initialDoubleAdvance":[
                  "h2",
                  "h4"
               ]
            }
         },
         {
            "turnNumber":5,
            "piece":{
               "type":"knight",
               "color":"black",
               "moved":true
            },
            "clientId":"196bfcc3-9613-4c9c-bd1b-da095ec0e75f",
            "movement":{
               "valid":[
                  "b8",
                  "c6"
               ]
            }
         },
         {
            "turnNumber":6,
            "piece":{
               "type":"pawn",
               "color":"white",
               "moved":true
            },
            "clientId":"fc98bd7c-9595-4e12-a4f9-6fb0e453e3c3",
            "movement":{
               "valid":[
                  "h4",
                  "h5"
               ]
            }
         },
         {
            "turnNumber":6,
            "piece":{
               "type":"bishop",
               "color":"black",
               "moved":true
            },
            "clientId":"196bfcc3-9613-4c9c-bd1b-da095ec0e75f",
            "movement":{
               "capture":[
                  "f8",
                  "e7"
               ]
            }
         },
         {
            "turnNumber":7,
            "piece":{
               "type":"pawn",
               "color":"white",
               "moved":true
            },
            "clientId":"fc98bd7c-9595-4e12-a4f9-6fb0e453e3c3",
            "movement":{
               "valid":[
                  "h5",
                  "h6"
               ]
            }
         },
         {
            "turnNumber":7,
            "piece":{
               "type":"knight",
               "color":"black",
               "moved":true
            },
            "clientId":"196bfcc3-9613-4c9c-bd1b-da095ec0e75f",
            "movement":{
               "valid":[
                  "c6",
                  "d4"
               ]
            }
         },
         {
            "turnNumber":8,
            "piece":{
               "type":"pawn",
               "color":"white",
               "moved":true
            },
            "clientId":"fc98bd7c-9595-4e12-a4f9-6fb0e453e3c3",
            "movement":{
               "capture":[
                  "h6",
                  "g7"
               ]
            }
         },
         {
            "turnNumber":8,
            "piece":{
               "type":"king",
               "color":"black",
               "moved":true
            },
            "clientId":"196bfcc3-9613-4c9c-bd1b-da095ec0e75f",
            "movement":{
               "castling":[
                  [
                     "e8",
                     "c8"
                  ],
                  [
                     "a8",
                     "d8"
                  ]
               ]
            }
         },
         {
            "turnNumber":9,
            "piece":{
               "type":"pawn",
               "color":"white",
               "moved":true
            },
            "clientId":"fc98bd7c-9595-4e12-a4f9-6fb0e453e3c3",
            "movement":{
               "capture":[
                  "g7",
                  "h8"
               ]
            }
         },
         {
            "turnNumber":9,
            "piece":{
               "type":"pawn",
               "color":"white",
               "moved":false
            },
            "clientId":"fc98bd7c-9595-4e12-a4f9-6fb0e453e3c3",
            "promotion":{
               "to":{
                  "type":"queen",
                  "color":"white",
                  "moved":false
               },
               "on":"h8"
            }
         }
      ],
      "whiteTimer":571541,
      "blackTimer":553672
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





