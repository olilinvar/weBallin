db = db.getSiblingDB('user-storage'); //schizoid-gpt

db.createCollection('gaming')

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