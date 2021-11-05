SELECT 'CREATE DATABASE mydb'
WHERE NOT EXISTS (SELECT FROM pg_database WHERE datname = 'mydb')\gexec