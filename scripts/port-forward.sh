#!/bin/bash
# Exit immediately if a command exits with a non-zero status.
set -e

# Enable debug logging to print each command before executing it.
set -x

echo "Starting simultaneous port-forwards for PostgreSQL and Redis..."

# Forward PostgreSQL
echo "Starting port-forward for PostgreSQL: forwarding local port 5433 to service port 5432..."
kubectl port-forward svc/postgresql 5433:5432 &
POSTGRES_PID=$!
echo "PostgreSQL port-forward started with PID ${POSTGRES_PID}."

# Forward Redis
echo "Starting port-forward for Redis: forwarding local port 6380 to service port 6379..."
kubectl port-forward svc/redis-master 6380:6379 &
REDIS_PID=$!
echo "Redis port-forward started with PID ${REDIS_PID}."

# Trap exit signals to kill the background processes if this script is stopped.
trap "echo 'Stopping port forwards...'; kill $POSTGRES_PID $REDIS_PID; exit 0" SIGINT SIGTERM

echo "Port-forwarding is now running. Press Ctrl+C to exit."

# Wait indefinitely while port-forwards run in the background.
wait
