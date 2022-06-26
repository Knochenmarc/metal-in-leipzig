<?php

declare(strict_types=1);

namespace MetalLE\Image;

use MetalLE\Event\Image;
use MetalLE\Site\Crawler;

class Optimizer
{
    private const LOCAL_DIR = '/public/';

    public function __construct(
        private Crawler $http = new Crawler(),
    ) {
    }

    public function optimize(Image $image): Image
    {
        if (file_exists(self::LOCAL_DIR . $image->publicJpgUrl)) {
            $this->decodeSize($image);
        } else {
            $this->convertThumbs($image);
        }

        return $image;
    }

    private function decodeSize(Image $image): void
    {
        [$width, $height] = getimagesize(self::LOCAL_DIR . $image->publicJpgUrl);
        if ($width > 0 && $height > 0) {
            $image->setSize((int) $width, (int) $height);
        }
    }

    private function convertThumbs(Image $image): void
    {
        $rawData = $this->http->get($image->remoteUrl);
        if ($rawData) {
            $tmpFile = tempnam('/tmp', $image->hash);
            if (false === file_put_contents($tmpFile, $rawData)) {
                throw new \Exception('could not write file: ' . $tmpFile);
            }

            shell_exec('convert -resize 300 -strip ' . $tmpFile . ' ' . self::LOCAL_DIR . $image->publicJpgUrl);
            shell_exec('convert -resize 300 -strip -define heic:speed=2 ' . $tmpFile . ' ' . self::LOCAL_DIR . $image->publicAvifUrl);
            unlink($tmpFile);

            $this->decodeSize($image);
        }
    }
}