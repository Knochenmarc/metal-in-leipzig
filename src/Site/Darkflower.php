<?php

declare(strict_types=1);

namespace MetalLE\Site;

use JsonException;
use MetalLE\Event\Event;
use MetalLE\Event\Location;
use Traversable;

class Darkflower implements Site
{
    use LinkedDataParser;

    /**
     * @throws JsonException
     */
    public function getIterator(): Traversable
    {
        $location = new Location('df', 'Darkflower', 'https://darkflower.club/');

        $http = new Crawler();
        $json = $http->get('https://darkflower.club/wp-json/wp/v2/pages/932');
        $json = json_decode($json, false, 512, JSON_THROW_ON_ERROR);
        foreach ($this->iterateEvents($json->content->rendered) as $event) {
            yield new Event(
                html_entity_decode(html_entity_decode($event->name)),
                new \DateTimeImmutable($event->startDate),
                $location,
                $event->url,
                $event->image,
                $event->{"@id"},
            );
        }
    }

}