# Vtuber Quiz Backend

## API list

- `POST /api/user`
    - description: create new user
    - hCaptcha: required
- `GET /api/user/self`
    - description: get self information
    - hCaptcha: no
- `POST /api/user/{username}/session`
    - description: login (create new session)
    - hCaptcha: required
- `POST /api/user/self/bilbili/verify_code`
    - description: create new bilbili challenge code
    - hCaptcha: no
- `POST /api/user/by-id/{id}/follow`
    - description: follow other user
    - hCaptcha: no
- `DELETE /api/user/by-id/{id}/follow`
    - description: unfollow other user
    - hCaptcha: no
- `POST /api/question`
    - description: create new question
    - hCaptcha: required
- `GET /api/question/{qid}`
    - description: get a question by id
    - hCaptcha: no
- `DELETE /api/question/{qid}`
    - description: delete a question by id
    - hCaptcha: no