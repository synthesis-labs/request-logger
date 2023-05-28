/*
  Warnings:

  - You are about to drop the column `request_body` on the `http_requests` table. All the data in the column will be lost.
  - You are about to drop the column `response_body` on the `http_requests` table. All the data in the column will be lost.

*/
-- AlterTable
ALTER TABLE "http_requests" DROP COLUMN "request_body",
DROP COLUMN "response_body";
