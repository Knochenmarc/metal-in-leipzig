<?php

declare(strict_types=1);

namespace MetalLE\Site;

interface Site
{
    public function crawl(): Location;
}