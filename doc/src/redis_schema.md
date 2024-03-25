# Redis Data Schema

| Name                                               | Type                  | Key Example                             | Expiration                | Module                                                                  |
| -------------------------------------------------- | --------------------- | --------------------------------------- | ------------------------- | ----------------------------------------------------------------------- |
| [Cookie](#cookie)                                  | **String(string)**    | `{cookie_token}`                        | 1 day (redis)             | `tower_sessions_redis_store`                                            |
| [Upload request](#upload-request)                  | **String(timestamp)** | `upload_request:{user_id}:{object_key}` | 1 hour (cron)             | `crate::domain::upload_request`                                         |
| [User candidate reg](#user-candidate-registration) | **HASH**              | `user_candidate:{email}`                | 30 min (redis)            | `crate::domain::user_candidate`                                         |
| [Username status req limit](#request-limit)        | **String(int)**       | `username_status_req_limit:{ip_addr}`   | 1 min or 12 hours (redis) | `crate::routes::development`, `crate::auth::login`, `crate::middleware` |
| [Payment](#payment)                                | **String(json)**      | `payment:{payment_id}`                  | No                        |                                                                         |

### Cookie

Cookie user auth data, body example:

```json
"\x93...\xb2\x81\xafaxum-login.data...\x00"
```

### Upload request

This upload request don't exire in redis, but in the `crate::startup::tasks` module,
at the `check_current_user_uploads` cron task, which runs every hour, we check redis for
outdated upload requests. We should do it in that task because we want to check object storage
for dangling files, and clean them.

Body example:

```json
"2024-03-14T14:06:10.602214+03:00"
```

### User candidate registration

Body example:

```json
{
  "username": "anyuser"
  "role": "consumer"
  "validation_token": "NMWWB347ZGz5yaOAzWCi5DpoJ"
  "password_hash": "MhzJtkBVGQ$WwDZ0fnNhlJ+CwQVPzR0Q2efwu1g"
  "email": "anyemail@anyhost.ru"
}
```

### Request limit

Body example:

```json
16
```

### Payment

Body example:

```json
{
  "id": "e8ef9831-0279-4495-973b-c49d73dcaee0",
  "user_id": 1,
  "amount": 1000,
  "beneficiaries": null
}
```
