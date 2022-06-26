<?php

declare(strict_types=1);

namespace MetalLE\Site\Shop;

use MetalLE\Site\Crawler;

class Mawi implements ShopCrawler
{
    public function __construct(
        private string $location,
        private Crawler $http = new Crawler(),
    ) {
    }

    public function fetchDates(): iterable
    {
        $html = $this->http->get('https://www.mawi-concert.de/index.php?menus_id=2', false);
        if (preg_match_all(
            '#class=\'grid-item framepic 15\' data-category=\'\'>\s*<a href=\'index\.php\?menus_id=2&solo=1&id=(\d+)\'#iU',
            $html,
            $matches
        )) {
            $eventIds = array_flip($matches[1]);

            $html = $this->http->post(
                'https://www.mawi-concert.de/worker/searching_inc.php',
                ['sk' => $this->location . '##location##'],
                false
            );
            if (preg_match_all(
                '#index.php\?menus_id=2&solo=1&id=(\d+)&.*am (\d\d\.\d\d\.\d\d\d\d)#isU',
                $html,
                $matches,
                PREG_SET_ORDER
            )) {
                foreach ($matches as $match) {
                    if (isset($eventIds[$match[1]])) {
                        yield new \DateTimeImmutable($match[2]);
                    }
                }
            }
        }
    }
}