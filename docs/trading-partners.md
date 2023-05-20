# Trading Partners

This document details the trading partner details used by the server for communication.

A trading partner record contains fields to lookup and verify the identity of a trading partner.

## Partner Definition

A trading partner is defined as a record and contains the following fields:

- `id` represents the record identifier (database primary key).
- `code` represents a friendly code to lookup this supplier, must be unique in instance.
- `name` represents the friendly name for this trading partner.
- `identifier` friendly identifier for this partner, must be unique.
- `gln` represents the **Global Location Number** for the sender or receiver
- `san` represents the alternative **Store Address Number** used in the UK
- `certificates` represents the certificate thumbprints for this trading partner public key

A trading partner is defined for both the receive and send side of a relationship

### Example

```json
{
  "id": 1,
  "code": "GBD001",
  "name": "Gruffs Book Distributors",
  "identifiers": [{ "gln": "500123457689" }, { "san": "234123X" }],
  "certificates": [
    {
      "as2": "SHA256 Fingerprint=75:75:EB:CD:C1:F8:A7:F9:9C:C7:0D:AC:5D:13:26:49:34:49:08:FD:D4:B7:06:DE:C7:83:CF:72:6F:D8:F8:60"
    }
  ]
}
```

#### Certificate Routes

To enable discovery of trading partners and easy access to the partner public key they can be
accessed via the following public routes.

The routes will return the public certificate if known or `404` **Not Found**

```http
# Get the partner AS2 certificate using custom identifier
GET /partner/code/GBD001/certs/as2
# Get the partner AS2 certificate using the customer Global Location Number (GLN)
GET /partner/gln/500123457689/certs/as2
```
