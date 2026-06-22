# Bootc Status API Reference

**Base URL:** `http://<host>:8005`

**Swagger UI:** `http://<host>:8005/swagger-ui/`

**OpenAPI Spec:** `http://<host>:8005/api-docs/openapi.json`

---

## Endpoints

### GET /health

Health check endpoint.

**Response 200:**
```json
{
  "status": "ok"
}
```

---

### GET /api/v1/status

Returns the full bootc host status, including booted, staged, and rollback entries.

**Response 200:**
```json
{
  "apiVersion": "org.containers.bootc/v1",
  "kind": "BootcHost",
  "metadata": {
    "name": "host"
  },
  "spec": {
    "bootOrder": "default",
    "image": {
      "image": "quay.io/example/my-image:v1.1",
      "transport": "registry"
    }
  },
  "status": {
    "booted": {
      "cachedUpdate": null,
      "composefs": null,
      "downloadOnly": false,
      "image": {
        "architecture": "amd64",
        "image": {
          "image": "quay.io/example/my-image:v1.1",
          "transport": "registry"
        },
        "imageDigest": "sha256:abc123...",
        "timestamp": "2026-06-17T23:41:39Z",
        "version": "10.2"
      },
      "incompatible": false,
      "ostree": {
        "checksum": "199cf803...",
        "deploySerial": 0,
        "stateroot": "default"
      },
      "pinned": false,
      "softRebootCapable": true,
      "store": "ostreeContainer"
    },
    "rollback": { "..." },
    "rollbackQueued": false,
    "staged": null,
    "type": "bootcHost",
    "usrOverlay": null
  }
}
```

**Response 500:** `Failed to execute bootc` or parse error message.

---

### GET /api/v1/status/booted

Returns the currently booted image details.

**Response 200:**
```json
{
  "cachedUpdate": null,
  "composefs": null,
  "downloadOnly": false,
  "image": {
    "architecture": "amd64",
    "image": {
      "image": "quay.io/example/my-image:v1.1",
      "transport": "registry"
    },
    "imageDigest": "sha256:abc123...",
    "timestamp": "2026-06-17T23:41:39Z",
    "version": "10.2"
  },
  "incompatible": false,
  "ostree": {
    "checksum": "199cf803...",
    "deploySerial": 0,
    "stateroot": "default"
  },
  "pinned": false,
  "softRebootCapable": true,
  "store": "ostreeContainer"
}
```

**Response 404:** `No booted entry`

**Response 500:** Command execution or parse error.

---

### GET /api/v1/status/staged

Returns details about a staged update (pending next reboot), if any.

**Response 200:** Same shape as the booted entry above.

**Response 404:** `No staged update`

**Response 500:** Command execution or parse error.

---

### GET /api/v1/status/rollback

Returns the rollback image entry.

**Response 200:** Same shape as the booted entry above.

**Response 404:** `No rollback entry`

**Response 500:** Command execution or parse error.

---

### GET /api/v1/status/update-available

Returns whether an update is available for the booted image. An update is considered available if there is a `cachedUpdate` on the booted entry or a `staged` entry exists.

**Response 200:**
```json
{
  "update_available": true,
  "current_image": {
    "image": "quay.io/example/my-image:v1.1",
    "transport": "registry"
  },
  "current_version": "10.2",
  "update_image": {
    "image": "quay.io/example/my-image:v1.2",
    "transport": "registry"
  },
  "update_version": "10.3"
}
```

When no update is available:
```json
{
  "update_available": false,
  "current_image": {
    "image": "quay.io/example/my-image:v1.1",
    "transport": "registry"
  },
  "current_version": "10.2",
  "update_image": null,
  "update_version": null
}
```

**Response 500:** Command execution or parse error.

---

## Error Responses

All error responses return a plain text body with the error message:

| Status | Meaning |
|--------|---------|
| 404 | Requested entry (booted/staged/rollback) does not exist |
| 500 | `bootc status --json` failed to execute or returned unparseable output |

---

## Deployment

1. Copy the binary to `/usr/local/bin/bootc-api`
2. Copy `bootc-api.service` to `/etc/systemd/system/`
3. Enable and start:
   ```bash
   sudo systemctl daemon-reload
   sudo systemctl enable --now bootc-api
   ```
4. Verify: `curl http://localhost:8005/health`
