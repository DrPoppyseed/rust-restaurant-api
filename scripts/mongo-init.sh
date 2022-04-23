mongosh -- "$MONGO_INITDB_DATABASE" <<EOF
  const rootUser = "$MONGO_INITDB_ROOT_USERNAME";
  const rootPassword = "$MONGO_INITDB_ROOT_PASSWORD";
  const adminDatabase = db.getSiblingDB("admin");
  adminDatabase.auth(rootUser, rootPassword);

  const user = "$MONGO_INITDB_USERNAME";
  const password = "$MONGO_INITDB_PASSWORD";
  db.createUser({
    user, pwd: password, roles: [{ role: "readWrite", db: "restaurant" }]
  });

  db = db.getSiblingDB("restaurant");
  db.createCollection("tables");
EOF
