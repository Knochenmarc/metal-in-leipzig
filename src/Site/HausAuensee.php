<?php

declare(strict_types=1);

namespace MetalLE\Site;

use MetalLE\Event\Event;
use MetalLE\Event\Location;
use Traversable;

class HausAuensee implements Site
{
    private const URL = 'https://haus-auensee-leipzig.de';

    public function getIterator(): Traversable
    {
        $eventim  = new Eventim();
        $location = new Location('ha', 'Haus Auensee', self::URL);

        $plainHtml = file_get_contents(self::URL);
        if (preg_match('#<div class="md-col md-col-8">.*</a>\s+</div>#isU', $plainHtml, $matches)) {
            $plainHtml = $matches[0];
        }

        foreach ($eventim->fetchDates('haus-auensee-leipzig-7301') as $date) {
            if (preg_match_all('#<a href="(.*)".*</a>#isU', $plainHtml, $matches, PREG_SET_ORDER)) {
                foreach ($matches as $match) {
                    if (str_contains($match[0], $date->format('>d.<'))
                        && str_contains($match[0], $date->format('>m.y<'))
                        && preg_match('#<h3.*>(.*)</h3>#iU', $match[0], $subMatches)
                    ) {
                        $imgUrl  = null;
                        $url = self::URL .'/'. $match[1];
                        $subPage = file_get_contents($url);
                        if (preg_match('#<img src="(.*)".*class="block col-12"#iU', $subPage, $imgMatch)) {
                            $imgUrl = self::URL . $imgMatch[1];
                        }

                        $evt = new Event(html_entity_decode($subMatches[1]), $date, $location, $url, $imgUrl);
                        yield $evt->getID() => $evt;
                        break;
                    }
                }
            }
        }
    }
}