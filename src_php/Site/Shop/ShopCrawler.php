<?php

declare(strict_types=1);

namespace MetalLE\Site\Shop;

interface ShopCrawler
{
    public function fetchDates(): iterable;
}