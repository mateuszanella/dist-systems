# Build stage
FROM golang:1.24-alpine AS builder

WORKDIR /app

COPY go.mod go.sum ./
RUN go mod download

COPY . .

RUN CGO_ENABLED=0 GOOS=linux go build -o /api ./cmd/api
RUN CGO_ENABLED=0 GOOS=linux go build -o /worker ./cmd/worker

FROM alpine:latest

RUN addgroup -S appgroup && adduser -S appuser -G appgroup

WORKDIR /app

COPY --from=builder /api /app/api
COPY --from=builder /worker /app/worker

EXPOSE 8080

USER appuser

# The default command will be to start the API. 
# You can override this in docker-compose to run the worker.
CMD ["./api"]
