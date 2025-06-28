# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Howitt is a web application for planning and tracking cycling/bikepacking routes in Australia. It features route management, ride tracking, water source locations, trip documentation, and integration with RideWithGPS.

## Architecture Overview

### Technology Stack
- **Backend**: Rust with Axum web framework, GraphQL (async-graphql), PostgreSQL with PostGIS
- **Frontend**: React 19 with Remix v2/Vite, TypeScript, Apollo Client, OpenLayers for maps
- **API Gateway**: TypeScript server using Bun and Hono framework
- **Infrastructure**: AWS CDK (S3, CloudFront, DynamoDB), Kubernetes
- **Runtime**: Bun for JavaScript/TypeScript services

### Project Structure
- `src/bin/` - Rust binaries (web server, worker, CLI)
- `src/lib/` - Rust libraries containing core business logic
  - `howitt/` - Core domain logic
  - `howitt-postgresql/` - Database layer with PostGIS integration
  - `rwgps/` - RideWithGPS API client
- `webui/` - React/Remix frontend application
- `ts-api/` - TypeScript API gateway for water features
- `cdk/` - AWS infrastructure as code
- `k8s/` - Kubernetes manifests
- `data/` - GPX routes, GTFS data, and other data files

### Key Concepts
- **Routes**: GPX-based cycling routes with elevation profiles and difficulty ratings
- **Water Sources**: Critical POIs for bikepacking with location and reliability info
- **Trips**: Multi-day journeys combining multiple routes with media and notes
- **RWGPS Integration**: Syncs routes and rides from RideWithGPS accounts

## Common Development Commands

### Frontend (webui)
```bash
cd webui
bun run dev          # Start development server (port 3000)
bun run build        # Build for production
bun run typecheck    # Run TypeScript type checking
bun run lint         # Run ESLint
bun run gql-codegen  # Regenerate GraphQL types after schema changes
bun run gql-watch    # Watch mode for GraphQL changes
bun run format       # Format code with Prettier
```

### TypeScript API (ts-api)
```bash
cd ts-api
bun run dev    # Start on port 3001
bun run serve  # Start on port 80
tsc --noEmit   # Type checking (use directly, never with npx)
```

### Rust Backend
```bash
# Run the main web server
cargo run --bin howitt-web

# Run the background worker
cargo run --bin howitt-worker

# Run CLI commands
cargo run --bin howitt-cli -- [command]

# Run tests
cargo test
cargo test -- --test-threads=1  # For tests requiring database isolation
cargo test [testname]            # Run specific test

# Code quality
cargo fmt                        # Format code
cargo clippy                     # Linting
cargo update                     # Update dependencies

# Database operations
cargo run --bin howitt-cli -- migrate  # Run migrations
```

### Infrastructure
```bash
# AWS CDK
cd cdk
npm run cdk synth    # Synthesize CloudFormation
npm run cdk deploy   # Deploy stack
npm run cdk diff     # Compare with deployed

# Kubernetes
kubectl apply -f k8s/
kubectl get deployments
kubectl logs -f deployment/howitt-web-api
```

## Frontend Development (React/Remix)

### Technology Stack
- **Framework**: Remix v2 with Vite
- **Runtime**: Bun (not Node.js)
- **UI Library**: React 19
- **State Management**: Apollo Client for GraphQL, React Query for REST, use-mutative for local state
- **Styling**: Emotion (@emotion/react) with CSS-in-JS + Tailwind CSS
- **Maps**: OpenLayers (ol)
- **Build Tool**: Vite
- **GraphQL**: Apollo Client with code generation
- **Form Handling**: react-hook-form
- **Icons**: Ionicons

### Project Structure
```
webui/
├── app/                      # Main application code
│   ├── __generated__/        # GraphQL generated types (DO NOT EDIT)
│   ├── components/           # React components
│   │   ├── layout/          # Layout components (Nav, containers)
│   │   ├── map/             # Map-related components and hooks
│   │   ├── pois/            # POI (Point of Interest) components
│   │   ├── rides/           # Ride-related components
│   │   ├── routes/          # Route-related components
│   │   ├── settings/        # Settings page components
│   │   ├── trips/           # Trip-related components
│   │   └── ui/              # Reusable UI components
│   ├── routes/              # Remix routes (file-based routing)
│   ├── services/            # Utility services and helpers
│   ├── styles/              # Styling configuration
│   └── root.tsx             # Root application component
├── public/                  # Static assets
├── codegen.ts              # GraphQL code generation config
└── vite.config.ts          # Vite configuration
```

### GraphQL Usage
- **Code Generation**: Always run `bun run gql-codegen` after modifying GraphQL queries
- **Query Definition**: Use the `gql` template tag from `app/__generated__/gql`
- **Fragment Usage**: Define fragments for reusable query parts
- **Type Safety**: Generated types are in `app/__generated__/graphql.ts`

```typescript
import { gql } from "~/__generated__/gql";

const QUERY = gql(`
  query routeQuery($slug: ID!) {
    route(slug: $slug) {
      id
      name
      ...routeDetails
    }
  }
`);
```

### Styling Approach
The project uses a hybrid styling approach:
- **Emotion CSS-in-JS** for component-specific styles
- **Tailwind CSS** for utility classes
- **Design tokens** in `app/styles/tokens.ts` for consistent colors

```typescript
import { css } from "@emotion/react";
import { tokens } from "~/styles/tokens";

const containerCss = css`
  background-color: ${tokens.colors.grey50};
  padding: 20px;
`;

// Usage
<div css={containerCss} className="rounded-lg">
```

### Frontend Conventions
- **File-based routing**: Routes are defined in `app/routes/`
- **Route naming**: Use dots for nested routes (e.g., `routes.$slug.tsx`)
- **Component organization**: Modular structure grouped by feature
- **Hooks**: Co-located with components in `hooks/` subdirectories
- **Map Integration**: Primary map context via React Context
- **TypeScript**: Strict mode enabled, use `~/*` for `app/*` imports, never use `!` operator

## TypeScript API Gateway

### Overview
The ts-api is a lightweight TypeScript API gateway built with Bun and the Hono framework. It serves as a specialized API layer for water-related features, providing GeoJSON endpoints that integrate with the PostGIS database.

### Technology Stack
- **Runtime**: Bun (v1.2.10+)
- **Framework**: Hono (v4.7.7) - lightweight web framework
- **Type Safety**: Zod (v4.0.0-beta) for runtime validation
- **Pattern Matching**: ts-pattern (v5.7.0) for exhaustive pattern matching
- **Compression**: Custom polyfill for CompressionStream support

### API Endpoints

#### 1. GET /api/water-features
Returns all water features with observation counts as a GeoJSON FeatureCollection.

#### 2. GET /api/water-features/query
Returns nearby water features based on location with detailed water observations.
- **Query Parameters**:
  - `origin`: Required, format "lon,lat" (e.g., "149.123,-35.456")
  - `radius`: Optional, search radius in meters (default: 1000)
  - `limit`: Optional, maximum results (default: 100)

#### 3. GET /api/now
Simple health check endpoint returning current ISO timestamp.

### Database Integration
- Direct PostgreSQL/PostGIS connection using Bun's SQL template tag
- Uses `ST_AsGeoJSON()` for geometry conversion
- Uses `ST_Distance()` and `ST_DWithin()` for proximity searches
- Coordinates are in EPSG:4326 (WGS84)

## Rust Backend

### Binary Crates (`src/bin/`)
- **howitt-web**: Main GraphQL API server using Axum and async-graphql
- **howitt-worker**: Background job processor using Apalis for async tasks
- **howitt-cli**: Command-line interface for administrative tasks and data management

### Library Crates (`src/lib/`)
- **howitt**: Core domain logic, models, and business services
- **howitt-postgresql**: Database layer with SQLx and PostGIS integration
- **howitt-jobs**: Job queue abstractions using Apalis with Redis
- **howitt-clients**: External service clients (S3, Redis)
- **rwgps**: RideWithGPS API client
- **mapbox-geocoding**: Mapbox geocoding API client
- **open-meteo**: Weather data API client
- **exif**: EXIF data extraction from images
- **gtfs**: GTFS transit data processing
- **csaps**: Cubic spline interpolation for elevation smoothing

### GraphQL API Implementation
- **Framework**: Axum with async-graphql
- **Authentication**: JWT-based with session management
- **Data Loading**: Uses async-graphql DataLoader for N+1 query prevention
- **Extractors**: Custom Axum extractors for authentication
- **File Upload**: Multipart support for media uploads

### Background Jobs (Apalis)
- **Job Queue**: Apalis with Redis backend
- **Job Types**:
  - Media processing (image optimization, WebP conversion, HEIF support)
  - RWGPS synchronization (routes, rides, trips)
- **Error Handling**: Automatic retries with backoff

### Database Layer (howitt-postgresql)
- **ORM**: SQLx with compile-time checked queries
- **Spatial**: PostGIS extension for geographic operations
- **Migrations**: Refinery for schema management
- **Repository Pattern**: Each entity has a dedicated repository

### CLI Commands
```bash
# User management
cargo run --bin howitt-cli -- user create
cargo run --bin howitt-cli -- user list

# Route operations
cargo run --bin howitt-cli -- route import
cargo run --bin howitt-cli -- route export

# Database operations
cargo run --bin howitt-cli -- migrate
```

## Infrastructure

### AWS CDK Infrastructure
Located in `/cdk/` directory:

1. **HowittAPI Stack** (ap-southeast-2)
   - DynamoDB table with GSI
   - S3 photos bucket (public)
   - CloudFront distribution for tile caching
   - Edge Lambda for cache control headers

2. **HowittMedia Stack** (ap-southeast-4)
   - S3 media bucket with intelligent tiering
   - CloudFront distribution for media delivery
   - S3 backups bucket with lifecycle rules

### Kubernetes Infrastructure
Located in `/k8s/` directory:

#### Services
1. **howitt-web-api**: Main Rust-based GraphQL API (port 8000)
2. **howitt-ts-api**: TypeScript API service (port 80)
3. **howitt-webui**: React/Remix web frontend (port 80)
4. **howitt-worker**: Background job processor
5. **howitt-db-backup**: Daily PostgreSQL backup job

#### Ingress Configuration
- Uses Traefik as ingress controller
- cert-manager for Let's Encrypt SSL certificates
- Domains:
  - howittplains.net → webui
  - api.howittplains.net → web-api
  - ts-api.howittplains.net → ts-api

### Database Backup Strategy
- **Daily**: Every day at 3 AM Brisbane time, 30-day retention
- **Weekly**: Every Monday, 180-day retention
- **Monthly**: 1st of each month, Glacier storage

### CI/CD Pipeline
- **CDK Deployment**: Automatic on push to main
- **Kubernetes Deployment**: Matrix build for all services
- Docker images pushed to GitHub Container Registry

## Common Patterns & Conventions

### Authentication & Authorization
- OAuth2 with session-based authentication
- JWT tokens stored in HTTP-only cookies
- User profiles and permissions managed through GraphQL API
- Argon2 for password hashing

### GraphQL Conventions
- Frontend queries use generated types from `gql-codegen`
- Backend uses async-graphql with DataLoader pattern
- Schema-first approach with code generation
- Field-level authorization in resolvers

### Database Patterns
- PostgreSQL with PostGIS extension required
- Spatial queries for geographic features
- OSM (OpenStreetMap) data storage
- Separate tables for route/ride points to optimize performance
- Connection pooling with appropriate limits

### Testing
- **Rust**: Standard cargo test with test-case and insta for snapshots
- **Frontend**: Currently no tests (use React Testing Library when adding)
- **Database tests**: Use transactions for isolation
- **Integration tests**: Mock external APIs where possible

### Environment Variables
- **Development**: `.env` files (not committed)
- **Production**: Kubernetes secrets and AWS Parameter Store
- **Key variables**: 
  - DATABASE_URL, REDIS_URL
  - JWT_SECRET
  - AWS_ACCESS_KEY_ID, AWS_SECRET_ACCESS_KEY
  - RWGPS_CLIENT_ID, RWGPS_CLIENT_SECRET
  - MAPBOX_ACCESS_TOKEN

### Error Handling
- **Rust**: `thiserror` for error derivation, `anyhow` for propagation
- **TypeScript**: Zod for validation, pattern matching for exhaustive checks
- **GraphQL**: Proper error responses with extensions

### Security Considerations
- SQL injection prevention via SQLx
- Input validation with Serde and Zod
- File upload validation and sanitization
- CORS configured for specific origins
- No hardcoded credentials in code

## Important Notes & Warnings

### General
- **DO NOT** edit files in `__generated__` directories
- **DO NOT** commit generated files from `build/` directory
- **DO NOT** commit `.env` files
- **ALWAYS** handle database transactions explicitly
- **ALWAYS** run format/lint tools before committing

### Frontend Specific
- **DO NOT** use `npm` directly - use `bun` for all commands
- **DO NOT** use `!` non-null assertions in TypeScript
- **ALWAYS** run `gql-codegen` after GraphQL changes
- **ALWAYS** use absolute imports with `~` alias
- **PREFER** client-side data fetching over Remix loaders

### Rust Specific
- **ALWAYS** use `cargo fmt` before committing
- **ALWAYS** be mindful of N+1 queries in GraphQL
- **PREFER** `anyhow::Result` in application code
- Use domain-specific errors in library code

### Infrastructure Specific
- **Region Split**: CDK resources split between ap-southeast-2 and ap-southeast-4
- **Public Buckets**: Photos bucket is public - ensure proper content validation
- **No Staging Environment**: Changes go directly to production
- **Manual Database**: PostgreSQL and Redis are not managed by this infrastructure

## Development Workflow

### Local Development Setup
```bash
# Start PostgreSQL with PostGIS
docker run -d -p 5432:5432 -e POSTGRES_PASSWORD=postgres postgis/postgis

# Start Redis
docker run -d -p 6379:6379 redis

# Run migrations
cargo run --bin howitt-cli -- migrate

# Start all services
cargo run --bin howitt-web      # Terminal 1
cargo run --bin howitt-worker   # Terminal 2
cd webui && bun run dev         # Terminal 3
cd ts-api && bun run dev        # Terminal 4
```

### Port Forwarding for Kubernetes
```bash
kubectl port-forward service/howitt-web-api 8000:80
kubectl port-forward service/howitt-webui 3000:80
```

## Emergency Procedures

### Rollback Deployment
```bash
kubectl rollout history deployment/howitt-web-api
kubectl rollout undo deployment/howitt-web-api
```

### Database Recovery
1. Stop write traffic (scale down web-api and worker)
2. Download appropriate backup from S3
3. Restore to new database instance
4. Update DATABASE_URL secret
5. Restart services

### Emergency Scale Down
```bash
kubectl scale deployment/howitt-web-api --replicas=0
kubectl scale deployment/howitt-web-api --replicas=1  # Scale back up
```