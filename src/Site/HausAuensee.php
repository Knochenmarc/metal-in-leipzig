<?php

declare(strict_types=1);

namespace MetalLE\Site;

use MetalLE\Event\Event;
use MetalLE\Event\Location;
use MetalLE\Site\Shop\Eventim;
use Traversable;

class HausAuensee implements Site
{
    private const URL = 'https://haus-auensee-leipzig.de';

    public function getIterator(): Traversable
    {
        $eventim  = new Eventim('haus-auensee-leipzig-7301');
        $location = new Location('ha', 'Haus Auensee', self::URL);

        $http = new Crawler();
        $plainHtml = $http->get(self::URL);
        if (preg_match('#<div class="md-col md-col-8">.*</a>\s+</div>#isU', $plainHtml, $matches)) {
            $plainHtml = $matches[0];
        }

        if (preg_match_all('#<a href="(.*)".*</a>#isU', $plainHtml, $matches, PREG_SET_ORDER)) {
            foreach ($eventim->fetchDates() as $date) {
                foreach ($matches as $key => $match) {
                    if (str_contains($match[0], $date->format('>d.<'))
                        && str_contains($match[0], $date->format('>m.y<'))
                        && preg_match('#<h3.*>(.*)</h3>#iU', $match[0], $subMatches)
                    ) {
                        $imgUrl  = null;
                        $url     = self::URL . '/' . $match[1];
                        $subPage = $http->get($url);
                        if (preg_match('#<img src="(.*)".*class="block col-12"#iU', $subPage, $imgMatch)) {
                            $imgUrl = self::URL . $imgMatch[1];
                        }

                        yield new Event(html_entity_decode($subMatches[1]), $date, $location, $url, $imgUrl);

                        unset ($matches[$key]);
                        break;
                    }
                }
            }
        }
    }
}