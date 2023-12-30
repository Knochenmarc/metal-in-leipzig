<?php

declare(strict_types=1);

namespace MetalLE\Site;

use Generator;
use MetalLE\Event\Event;
use MetalLE\Event\Location;
use MetalLE\Site\Shop\Eventim;
use MetalLE\Site\Shop\LiveGigs;
use MetalLE\Site\Shop\ShopCrawler;
use Traversable;

class HausAuensee implements Site
{
    private const URL = 'https://haus-auensee-leipzig.de';

    public function __construct(
        private Location $location = new Location('ha', 'Haus Auensee', self::URL),
        private Crawler $http = new Crawler(),
    ) {
    }

    public function getIterator(): Traversable
    {
        $plainHtml = $this->http->get(self::URL);
        if (preg_match('#<div class="md-col md-col-8">.*</a>\s+</div>#isU', $plainHtml, $matches)) {
            $plainHtml = $matches[0];
        }

        if (preg_match_all('#<a href="(.*)".*</a>#isU', $plainHtml, $matches, PREG_SET_ORDER)) {
            yield from $this->filter(new Eventim('haus-auensee-leipzig-7301'), $matches);
            yield from $this->filter(new LiveGigs('Haus Auensee'), $matches);
        }
    }

    private function filter(ShopCrawler $shop, array &$matches): Generator
    {
        foreach ($shop->fetchDates() as $date) {
            foreach ($matches as $key => $match) {
                if (str_contains($match[0], $date->format('>d.<'))
                    && str_contains($match[0], $date->format('>m.y<'))
                    && preg_match('#<h3.*>(.*)</h3>#iU', $match[0], $subMatches)
                ) {
                    $imgUrl  = null;
                    $url     = self::URL . '/' . $match[1];
                    $subPage = $this->http->get($url);
                    if (preg_match('#<img src="(.*)".*class="block col-12"#iU', $subPage, $imgMatch)) {
                        $imgUrl = self::URL . $imgMatch[1];
                    }

                    yield new Event(html_entity_decode($subMatches[1]), $date, $this->location, $url, $imgUrl);

                    unset ($matches[$key]);
                    break;
                }
            }
        }
    }

    public function getLocations(): iterable
    {
        yield $this->location;
    }
}