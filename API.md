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

- Submit a new product:
```
→ TO
POST /api/protected/creator/submit_product
Example json:
{
  "Song": {
    "product": {
      "name": "some_song",
      "description": null,
      "moods": [
        "веселый"
      ],
      "cover_object_key": "Lenora Ryan-95c9a0bf-2391-42fb-823a-36eb124a643c-image.png",
      "price": "100"
    },
    "music_product": {
      "master_object_key": "Lenora Ryan-4c8295bd-dff5-45aa-a706-dcea503e5015-song.mp3",
      "master_tagged_object_key": null,
      "multitrack_object_key": "Lenora Ryan-424f5e87-3dd9-48d0-9c70-687f7a23e4d6-arch.zip",
      "primary_genre": "Хор",
      "secondary_genre": null,
      "tempo": 100,
      "duration": 30,
      "key": "a_major"
    },
    "lyric": "this is song's lyric. Is it long enough or not?",
    "sex": "Female"
  }
}
OR
{
  "Beat": {
    "product": {
      "name": "some_song",
      "description": null,
      "moods": [
        "веселый"
      ],
      "cover_object_key": "Jerel Schuppe-4e4f2d6f-3fe8-4d4c-a983-44b2c72c102c-image.png",
      "price": "100"
    },
    "music_product": {
      "master_object_key": "Jerel Schuppe-aa877066-116e-46c9-90cc-2f52674151a1-song.mp3",
      "master_tagged_object_key": null,
      "multitrack_object_key": "Jerel Schuppe-f2f0c47a-ab76-4247-a596-1ca97cf597b2-arch.zip",
      "primary_genre": "Хор",
      "secondary_genre": null,
      "tempo": 100,
      "duration": 30,
      "key": "a_major"
    }
  }
}
OR
{
  "Cover": {
    "product": {
      "name": "some_song",
      "description": null,
      "moods": [
        "веселый"
      ],
      "cover_object_key": "Maryse Bergstrom-4e81b68b-5bba-47ce-835b-794add89b9f4-image.png",
      "price": "100"
    }
  }
}
OR
{
  "Lyric": {
    "product": {
      "name": "some_song",
      "description": null,
      "moods": [
        "веселый"
      ],
      "cover_object_key": "Kaley Goodwin-002ef652-11ea-4923-8985-fce8c5ab3de7-image.png",
      "price": "100"
    },
    "lyric": "this is just lyric. Is it long enough or not?",
    "sex": null
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

- Submit a new service

```
→ TO
POST /api/protected/creator/submit_service
Example json:
{
  "Mixing": {
    "service": {
      "name": "Some service",
      "description": null,
      "cover_object_key": "Alfonso Johnson-ec0d2295-36e1-43f8-a309-2346bd4725be-image.png",
      "display_price": "500",
      "credits_object_keys": [
        "Alfonso Johnson-c0b0cdbe-5443-4e90-aa22-0ca81c08c747-song-0.mp3",
        "Alfonso Johnson-eec512ea-019a-48f5-9191-be873efee010-song-1.mp3",
        "Alfonso Johnson-de3bd615-1635-4ae3-9a41-f65a943ecebe-song-2.mp3"
      ]
    },
    "genres": []
  }
}
OR
{
  "SongWriting": {
    "service": {
      "name": "Some service",
      "description": null,
      "cover_object_key": "Leilani Kemmer-307e6f94-5a81-4e0a-9b4e-9c3fd7cc37f2-image.png",
      "display_price": "500",
      "credits_object_keys": null
    },
    "genres": [
      "Нью вейв",
      "Бас",
      "Джаз",
      "Национальный фолк"
    ]
  }
}
OR
{
  "BeatWriting": {
    "service": {
      "name": "Some service",
      "description": null,
      "cover_object_key": "Leilani Kemmer-307e6f94-5a81-4e0a-9b4e-9c3fd7cc37f2-image.png",
      "display_price": "500",
      "credits_object_keys": null
    },
    "genres": [
      "Нью вейв",
      "Бас",
      "Джаз",
      "Национальный фолк"
    ]
  }
}
OR
{
  "GhostWriting": {
    "service": {
      "name": "Some service",
      "description": null,
      "cover_object_key": "Leilani Kemmer-307e6f94-5a81-4e0a-9b4e-9c3fd7cc37f2-image.png",
      "display_price": "500",
      "credits_object_keys": null // ATTENTION! For ghost writing this should be null!
    },
    "credits": null
  }
}
OR
{
  "CoverDesign": {
    "name": "Some service",
    "description": null,
    "cover_object_key": "Leilani Kemmer-307e6f94-5a81-4e0a-9b4e-9c3fd7cc37f2-image.png",
    "display_price": "500",
    "credits_object_keys": null
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
