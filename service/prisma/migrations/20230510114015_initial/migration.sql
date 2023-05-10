-- CreateTable
CREATE TABLE "http_requests" (
    "id" TEXT NOT NULL,
    "app" TEXT NOT NULL DEFAULT '',
    "time_generated" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "username" TEXT NOT NULL DEFAULT '',
    "request_time_ms" INTEGER NOT NULL,
    "request_method" TEXT NOT NULL DEFAULT '',
    "request_uri" TEXT NOT NULL DEFAULT '',
    "request_body" TEXT NOT NULL DEFAULT '',
    "response_body" TEXT NOT NULL DEFAULT '',

    CONSTRAINT "http_requests_pkey" PRIMARY KEY ("id")
);
