# Services

This document details the services that are available to a trading partner.

## Service Definition

A service offered by a trading partner is defined as a record containing the following fields:

- `id` represents the service record identifier (database primary key).
- `name` represents the friendly name for this service.

## Service Routes

### Send File to partner

```http
# Send a file to the configured destination for this partner
POST /partner/id/GBD001
```
