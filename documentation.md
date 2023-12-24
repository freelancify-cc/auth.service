GET /api/user/ 
    - {
        "status": "success",
        "data": {
            "users": [
                {
                    "id": "24bf36d6-6fa6-4bd9-b3a5-c820a171c350",
                    "email": "rxored@gmail.com",
                    "username": "rxored",
                    "first_name": "jayod",
                    "second_name": "bandara",
                    "profile_picture_url": "adadd",
                    "date_of_birth": "2023-11-22 15:30:00",
                    "role_name": "freelancer"
                },
                {
                    "id": "24bf36d6-6fa6-4bd9-b3a5-c820a171c350",
                    "email": "rxored@gmail.com",
                    "username": "niflan",
                    "first_name": "idk",
                    "second_name": "niflan",
                    "profile_picture_url": "https://pcire",
                    "date_of_birth": "2004-01-29",
                    "role_name": "freelancer"
                },
                {
                    "id": "24bf36d6-6fa6-4bd9-b3a5-c820a171c350",
                    "email": "rxored@gmail.com",
                    "username": "rxored",
                    "first_name": "adadadada",
                    "second_name": "b",
                    "profile_picture_url": "https://pcire",
                    "date_of_birth": "2004-01-29",
                    "role_name": "freelancer"
                },
                {
                    "id": "7acdc590-8e38-4379-8474-f3288e3fac15",
                    "email": "testuser@gmail.com",
                    "username": "testuser",
                    "first_name": "test",
                    "second_name": "user",
                    "profile_picture_url": "profile pic is here",
                    "date_of_birth": "2023-11-22 15:30:00",
                    "role_name": "employer"
                },
                {
                    "id": "7acdc590-8e38-4379-8474-f3288e3fac15",
                    "email": "testuser@gmail.com",
                    "username": "testuser2",
                    "first_name": "test2",
                    "second_name": "user2",
                    "profile_picture_url": "profile pic is here",
                    "date_of_birth": "2023-11-22 15:30:00",
                    "role_name": "employer"
                },
                {
                    "id": "a2f45e8b-e8a4-40cf-98c6-a5ad52ff06c1",
                    "email": "testuser1@gmail.com",
                    "username": "testuser2",
                    "first_name": "test2",
                    "second_name": "user2",
                    "profile_picture_url": "profile pic is here",
                    "date_of_birth": "2023-11-22 15:30:00",
                    "role_name": "employer"
                }
            ]
        }
    }


GET /api/user/id
    - response 
    {
        "status": "success",
        "data": {
            "users": {
                "id": "a2f45e8b-e8a4-40cf-98c6-a5ad52ff06c1",
                "email": "testuser1@gmail.com",
                "username": "testuser2",
                "first_name": "test2",
                "second_name": "user2",
                "profile_picture_url": "profile pic is here",
                "date_of_birth": "2023-11-22 15:30:00",
                "role_name": "employer"
            }
        }
    }

POST /api/user/register 
    - request 
    {
        "email": "testuser1@gmail.com",
        "password": "test@123",
        "role": 2
    }

    - response 
    {
        "status": "success",
        "data": {
            "user": {
                "id": "1107eb26-1518-4046-a223-a6d6e5332506",
                "email": "testuser2@gmail.com",
                "role": 2
            }
        }
    }

POST /api/user/create_profile
    - request 
    {
        "first_name": "test2",
        "second_name": "user2",
        "id": "a2f45e8b-e8a4-40cf-98c6-a5ad52ff06c1",
        "username": "testuser2",
        "date_of_birth": "2023-11-22 15:30:00",
        "profile_picture_url": "profile pic is here" 
    }

    - response 
    {
        "status": "success",
        "data": {
            "profile": {
                "username": "testuser2",
                "first_name": "test2",
                "second_name": "user2",
                "profile_picture_url": "profile pic is here"
            }
        }
    }


POST /api/auth/login
    - r3equest 
    {
        "email": "testuser1@gmail.com",
        "password": "test@123"
    }

    - response 
    {
        "status": "success",
        "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJhMmY0NWU4Yi1lOGE0LTQwY2YtOThjNi1hNWFkNTJmZjA2YzEiLCJpYXQiOjE3MDM0MzAxNzMsImV4cCI6MTcwMzQzMzc3M30.46xWk2ib8YKx5VMylvcc9WkL9ZoClufjolNfRnfRJBc"
    }

