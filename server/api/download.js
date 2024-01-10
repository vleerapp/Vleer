// server/api/download.js
import { defineEventHandler, createError } from 'h3';
import ytdl from 'ytdl-core';
import { URL } from 'url';

export default defineEventHandler(async (event) => {
  const url = new URL(event.req.url, `http://${event.req.headers.host}`).searchParams.get('url');

  if (!url) {
    throw createError({ statusCode: 400, statusMessage: 'No URL provided' });
  }

  try {
    // Verify the URL is a valid YouTube URL
    const valid = ytdl.validateURL(url);
    if (!valid) {
      throw createError({ statusCode: 400, statusMessage: 'Invalid YouTube URL' });
    }

    // Get video info
    const info = await ytdl.getInfo(url);
    const title = info.videoDetails.title;

    // Set headers to prompt download on the client side
    event.res.writeHead(200, {
      'Content-Type': 'audio/webm',
      'Content-Disposition': `attachment; filename="${title}.webm"`,
    });

    const audioStream = ytdl(url, { quality: 'highestaudio', filter: 'audioonly' });

    // Pipe the audio stream directly to the response
    audioStream.pipe(event.res);
  } catch (error) {
    // Handle errors
    console.error(error);
    throw createError({ statusCode: 500, statusMessage: 'Internal Server Error' });
  }
});