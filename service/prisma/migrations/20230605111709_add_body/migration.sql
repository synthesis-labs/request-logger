-- CreateTable
CREATE TABLE "http_request_data" (
    "id" TEXT NOT NULL,
    "request_data_json" TEXT NOT NULL,
    "response_data_json" TEXT NOT NULL,
    "http_request_id" TEXT NOT NULL,

    CONSTRAINT "http_request_data_pkey" PRIMARY KEY ("id")
);

-- CreateIndex
CREATE UNIQUE INDEX "http_request_data_http_request_id_key" ON "http_request_data"("http_request_id");

-- AddForeignKey
ALTER TABLE "http_request_data" ADD CONSTRAINT "http_request_data_http_request_id_fkey" FOREIGN KEY ("http_request_id") REFERENCES "http_requests"("id") ON DELETE RESTRICT ON UPDATE CASCADE;
