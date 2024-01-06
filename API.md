# MUSTORE API

### Authorization

- User signup:
```
→ TO
POST /api/signup
Form: {
    username: String,
    password: String,
    email: String,
    user_role: UserRole ('creator', 'consumer'), OPTIONAL
    admin_token: Uuid, OPTIONAL
}
← FROM
If OK, StatusCode::Created (201)
if Err, 1. If internal error > StatusCode::InternalError (500)
        2. If bad input (for example if user already exists) > StatusCode::BadRequest (400) 
```
> When user signs up, a confirmation link is sent to the provided email address. This link is valid for 30 minutes. 

- User login:
```
→ TO
POST /api/login
Json: {
    username: String,
    password: String,
}
← FROM
If OK, StatusCode::OK (200)
if Err, 1. If internal error > StatusCode::InternalError (500)
        2. If bad input > StatusCode::Unautorized (401) 
```

- User logout:
```
→ TO
GET /api/logout
← FROM
If OK, browser will be redirected (we should make decision where to)
if Err > StatusCode::InternalError (500)
```

- Check user permissions:
```
→ TO
GET /api/protected/health_check
    OR /api/protected/user/health_check
    OR /api/protected/admin/health_check
    OR /api/protected/creator/health_check
    OR /api/protected/consumer/health_check
← FROM
If OK > StatusCode::OK (200)
if Err > StatusCode::Forbidden (403)
```

### Open routes

- Get beats, songs, covers, lyrics COUNTS:
```
→ TO
GET /api/open/stats
← FROM
If OK, Json {
    songs: number,
    beats: number,
    covers: number,
    lyrics: number,
}, StatusCode::OK (200)
if Err > StatusCode::InternalError (500)
```

- Get genres or tags list:
```
→ TO
GET /api/open/genres OR GET /api/open/tags
← FROM
If OK, Json [..list], StatusCode::OK (200)
if Err:
    1. If internal error > StatusCode::InternalError (500)
    2. If bad input > StatusCode::BadRequest (400) 
```

- Get songs objects:
```
→ TO
GET /api/open/songs
Json: {
    sex: Option<Sex>,
    tempo: Option<Vec<i16>>,
    key: Option<Vec<MusicKey>>,
    genres: Option<Vec<String>>,
    vibes: Option<Vec<String>>,
    sort_by: SortBy,
    amount: i64,
}
← FROM
If OK, Json [..list], StatusCode::OK (200)
if Err:
    1. If internal error > StatusCode::InternalError (500)
    2. If bad input > StatusCode::BadRequest (400) 
```

- Get new songs:
```
→ TO
GET /api/open/new_songs?amount=<arg>
← FROM
If OK, Json [..list], StatusCode::OK (200)
if Err,
    1. If internal error > StatusCode::InternalError (500)
    2. If bad input > StatusCode::BadRequest (400) 
```
- Get recommended songs:
```
→ TO
GET /api/open/recommended_songs?amount=<arg>
← FROM
If OK, Json [..list], StatusCode::OK (200)
if Err:
    1. If internal error > StatusCode::InternalError (500)
    2. If bad input > StatusCode::BadRequest (400) 
```

### User routes

- Get presigned post form to upload file on object storage:
```
→ TO
GET /api/protected/user/req_upload_form?media_type=<media_type>&file_name=<file_name>

WHERE media_type is mime type, like 'image/png',
file_name is string.

← FROM
If OK, Json (complex structure, look at the source code), StatusCode::OK (200)
if Err:
    1. If internal error > StatusCode::InternalError (500)
    2. If bad input > StatusCode::NotAcceptable (406) 
    3. If don't have permission > StatusCode::Forbidden (403)
    4. If not authorized > StatusCode::Unautorized (401)
    5. If too many uploads for that user > StatusCode::TooManyRequests (429)
    6. If filename containes forbidden characters > StatusCode::BadRequest (400) + error description in the body.
```

### Creator routes

- Submit a new music product (song / beat):
```
→ TO
POST /api/protected/creator/submit_music_product
Example json:
{
  "Song": {
    "lyric": "this is song's lyric. Is it long enough or not?",
    "sex": "Female",
    "music_product": {
      "master_object_key": "Alycia Daniel-27402d05-a1f7-4562-a082-e6268ffe9d43-song.mp3",
      "master_tagged_object_key": null,
      "multitrack_object_key": "Alycia Daniel-e61dfa06-abb7-4ebd-98ab-2c9002a5e850-arch.zip",
      "cover_object_key": "Alycia Daniel-401ad043-994b-44f3-9458-2c0044625567-image.png",
      "name": "some_song",
      "description": null,
      "moods": [
        "веселый"
      ],
      "primary_genre": "Хор",
      "secondary_genre": null,
      "tempo": 100,
      "duration": 30,
      "price": "100",
      "key": "a_major"
    }
  }
}
OR
{
  "Beat": {
    "master_object_key": "Brandi Prosacco-cbc6fdc3-f11f-432f-85b5-0c5b50122e7e-song.mp3",
    "master_tagged_object_key": null,
    "multitrack_object_key": "Brandi Prosacco-421f1174-b2ce-494d-a7c0-b54d51a9a34d-arch.zip",
    "cover_object_key": "Brandi Prosacco-b1c79d98-1257-4299-8b81-38bb4d2ddf26-image.png",
    "name": "some_song",
    "description": null,
    "moods": [
      "веселый"
    ],
    "primary_genre": "Хор",
    "secondary_genre": null,
    "tempo": 100,
    "duration": 30,
    "price": "100",
    "key": "a_major"
  }
}
← FROM
If OK, StatusCode::CREATED (201)
if Err:
    1. If internal error > StatusCode::InternalError (500)
    2. If bad input > StatusCode::NotAcceptable (406) 
    3. If don't have permission > StatusCode::Forbidden (403)
    4. If not authorized > StatusCode::Unautorized (401)
    6. If filename containes forbidden characters > StatusCode::BadRequest (400) + error description in the body.
    7. If no upload registered in the redis cache, > StatusCode::ExpectationFailed (417)
```
