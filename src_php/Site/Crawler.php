<?php

declare(strict_types=1);

namespace MetalLE\Site;

class Crawler
{
    private const HEADERS = "User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:96.0) Gecko/20100101 Firefox/96.0\r\n"
    . "Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8\r\n"
    . "Accept-Language: de,en-US;q=0.7,en;q=0.3\r\n"
    . "Accept-Encoding: identity\r\n"
    . "DNT: 1\r\n"
    . "Connection: close\r\n"
    . "Pragma: no-cache\r\n"
    . "Cache-Control: no-cache\r\n";

    public function __construct(
        private bool $debug = true,
    ) {
    }

    public function get(string $url, bool $checkCert = true): bool|string
    {
        if ($this->debug) {
            print 'get: ' . $url . PHP_EOL;
        }

        return file_get_contents(
            $url,
            false,
            stream_context_create([
                'http' => ['method' => 'GET', 'header' => self::HEADERS],
                'ssl' => ['verify_peer' => $checkCert, 'verify_peer_name' => $checkCert,]
                                  ])
        );
    }

    public function post(string $url, array $payload, bool $checkCert = true): bool|string
    {
        if ($this->debug) {
            print 'post: ' . $url . PHP_EOL;
        }

        $options = [
            'method'  => 'POST',
            'header'  => self::HEADERS . "Content-Type: application/x-www-form-urlencoded\r\n",
            'content' => http_build_query($payload),
        ];

        return file_get_contents(
            $url,
            false,
            stream_context_create([
                'http' => $options,
                'ssl' => ['verify_peer' => $checkCert, 'verify_peer_name' => $checkCert,]
              ])
        );
    }
}