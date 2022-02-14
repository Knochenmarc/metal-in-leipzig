<?php

declare(strict_types=1);

namespace MetalLE\Site;

use MetalLE\Event\Event;
use MetalLE\Event\Location;
use MetalLE\Site\Shop\Eventim;
use Traversable;

class Taeubchenthal implements Site
{
    public function getIterator(): Traversable
    {
        $location = new Location('tt', 'Täubchenthal', 'https://www.taeubchenthal.com/');

        $http = new Crawler();
        $json = $http->get(
            'https://www.taeubchenthal.com/wp-admin/admin-ajax.php?id=&post_id=36&slug=programm&posts_per_page=100&page=0&offset=0&post_type=event&repeater=default&meta_key=evetndatum&meta_type=DATE&order=ASC&orderby=meta_value_num&action=alm_get_posts&query_type=standard'
        );
        $json = json_decode($json, false, 512, JSON_THROW_ON_ERROR);
        if (preg_match_all(
            "#<div class='event'><a href='(.*)'.*url\((.*)\).*<span>(\d\d\.\d\d\.\d\d\d\d)</span><h3>(.*)</h3>#iU",
            $json->html,
            $matches,
            PREG_SET_ORDER
        )) {
            $eventim = new Eventim('taeubchenthal-leipzig-18055');

            foreach ($eventim->fetchDates() as $date) {
                foreach ($matches as $match) {
                    if ($date->format('d.m.Y') === $match[3]) {
                        yield new Event(
                            $match[4],
                            $date,
                            $location,
                            $match[1],
                            $match[2],
                        );
                        continue 2;
                    }
                }
            }
        }
    }
}