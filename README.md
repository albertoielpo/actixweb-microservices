# TODO
```
GET
http://localhost:3000/rate
Type: application/json

SSE (Server sent event)
http://localhost:3000/sse
Type: text/event-stream

Websocket
http://localhost:3001
```
##
https ?
## DTO
export class RateDto {
    @ApiProperty()
    rate: number;
}
##
micro-rust
-- microservice scaffolding
-- actixweb

websocket-rust
-- websocket scaffolding
-- actixweb