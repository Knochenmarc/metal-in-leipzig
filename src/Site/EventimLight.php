<?php

declare(strict_types=1);

namespace MetalLE\Site;

class EventimLight
{
    public function fetchDates(string $venue): iterable
    {
        $customHeaders =
            "User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:96.0) Gecko/20100101 Firefox/96.0\r\n"
            ."Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8\r\n"
            ."Accept-Language: de,en-US;q=0.7,en;q=0.3\r\n"
            ."Accept-Encoding: identity\r\n"
            ."DNT: 1\r\n"
            ."Connection: close\r\n"
            ."Pragma: no-cache\r\n"
            ."Cache-Control: no-cache\r\n"
        ;
        $context = stream_context_create(['http' => ['method' => 'GET', 'header' => $customHeaders]]);

        $plainHtml = file_get_contents(
            'https://www.eventim-light.com/de/a/' . $venue,
            false,
            $context,
        );
        if (preg_match('#window.__INITIAL_STATE__=({.*});#iU', $plainHtml, $matches)) {
            $data = json_decode($matches[1], false, 512, JSON_THROW_ON_ERROR);
            foreach ($data->events->cache as $item) {
                if ($item->category === 'Hard & Heavy') {
                    yield $item->title => new \DateTimeImmutable($item->start);
                }
            }
        }

        return [];
    }
}