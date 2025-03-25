curl -H 'Content-Type: application/json' \
  -d '{
    "id": 1,
    "first_name": "Gael",
    "last_name": "Zarco",
    "email": "gaelxarco@icloud.com",
    "body": "Test"
  }' \
  -X POST http://127.0.0.1:5000/contact/email
