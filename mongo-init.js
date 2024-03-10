db = db.getSiblingDB('user-storage'); //schizoid-gpt

db.createCollection('users')
db.createCollection('highscores')

db.createUser({
  user: 'user',
  pwd: 'password',
  roles: [
    {
      role: 'readWrite',
      db: 'user-storage',
    }, 
  ],
});