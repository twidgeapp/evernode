-- CreateTable
CREATE TABLE "database" (
    "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    "name" TEXT NOT NULL,
    "connection_string" TEXT NOT NULL,
    "database_type" TEXT NOT NULL
);
