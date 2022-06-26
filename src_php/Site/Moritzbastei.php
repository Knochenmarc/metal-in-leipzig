<?php

declare(strict_types=1);

namespace MetalLE\Site;

use MetalLE\Event\Event;
use MetalLE\Event\Location;
use Traversable;

class Moritzbastei implements Site
{
    public function __construct(
        private Location $location = new Location('mb', 'Moritzbastei', 'https://www.moritzbastei.de/',),
    ) {
    }

    public function getIterator(): Traversable
    {
        $http = new Crawler();
        $response = $http->get('https://www.moritzbastei.de/',);
        if (preg_match('#"security":"([a-z0-9]+)"#iU', $response, $matches)) {
            $response = $http->post(
                'https://www.moritzbastei.de/wp-admin/admin-ajax.php?offset=0&limit=100',
                ['action' => 'event_ajax_action_callback', 'security' => $matches[1]]
            );
            $json     = json_decode($response, false, 512, JSON_THROW_ON_ERROR);

            foreach ($json->content as $html) {
                if (
                    str_contains($html, '#Metal')
                    && preg_match(
                        '#<img.*src="(.*)".*(\d\d\.\d\d\.\d\d\d\d).*<h3.*<a href="(.*)">(.*)</a>#iUs',
                        $html,
                        $matches
                    )
                ) {
                    yield new Event(
                        html_entity_decode($matches[4]),
                        new \DateTimeImmutable($matches[2]),
                        $this->location,
                        $matches[3],
                        str_replace('-100x100', '', $matches[1]),
                        $matches[3],
                    );
                }
            }
        }
    }

    public function getLocations(): iterable
    {
        yield $this->location;
    }
}