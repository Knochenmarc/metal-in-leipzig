<?php

declare(strict_types=1);

namespace MetalLE\Image;

use MetalLE\Event\Image;
use MetalLE\Site\Crawler;

class Optimizer
{
    public function __construct(
        private Crawler $http = new Crawler(),
    ) {
    }

    public function optimize(Image $image): Image
    {
        $localFilePath = '/public/' . $image->publicUrl;

        if (file_exists($localFilePath)) {
            $this->decodeSize($localFilePath, $image);
        } else {
            $this->convertToAvif($localFilePath, $image);
        }

        return $image;
    }

    private function decodeSize(string $localFilePath, Image $image): void
    {
        $shell = shell_exec('avifdec -i ' . $localFilePath);
        if (preg_match('#Resolution\s+: (\d+)x(\d+)#i', $shell, $matches)) {
            $image->setSize((int) $matches[1], (int) $matches[2]);
        }
    }

    private function convertToAvif(string $localFilePath, Image $image): void
    {
        $rawData = $this->http->get($image->remoteUrl);
        if ($rawData) {
            $tmpFile = '/tmp/' . $image->hash;
            file_put_contents($tmpFile, $rawData);
            shell_exec('convert -resize 300 -strip -define heic:speed=1 ' . $tmpFile . ' ' . $localFilePath);
            unlink($tmpFile);

            $this->decodeSize($localFilePath, $image);
        }
    }
}