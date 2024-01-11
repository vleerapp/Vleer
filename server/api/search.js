// server/api/search.js
import { defineEventHandler } from 'h3';
import axios from 'axios';

export default defineEventHandler(async (event) => {
  const query = event.req.url.split('?')[1];
  const params = new URLSearchParams(query);
  const searchTerm = params.get('term');

  if (!searchTerm) {
    return {
      error: 'No search term provided',
    };
  }

  const youtubeApiKey = 'AIzaSyBJ-mQ4fiKtnkMrEZBpCuzlXzIqvtmTsGc'; // Replace with your actual API key
  const youtubeSearchApiUrl = 'https://www.googleapis.com/youtube/v3/search';
  const youtubeVideosApiUrl = 'https://www.googleapis.com/youtube/v3/videos';

  try {
    // Search for videos on YouTube
    const searchResponse = await axios.get(youtubeSearchApiUrl, {
      params: {
        part: 'snippet',
        maxResults: 20,
        q: searchTerm,
        type: 'video',
        videoCategoryId: '10', // Category ID for Music
        key: youtubeApiKey,
      },
    });

    // Get video IDs from search results
    const videoIds = searchResponse.data.items.map(item => item.id.videoId).join(',');

    // Get statistics for each video
    const statsResponse = await axios.get(youtubeVideosApiUrl, {
      params: {
        part: 'statistics',
        id: videoIds,
        key: youtubeApiKey,
      },
    });

    // Create a map of video IDs to view counts
    const viewCounts = statsResponse.data.items.reduce((acc, item) => {
      acc[item.id] = item.statistics.viewCount;
      return acc;
    }, {});

    // Combine search results with view counts and sort by view count
    const resultsWithViewCount = searchResponse.data.items.map((item) => ({
      title: item.snippet.title,
      videoId: item.id.videoId,
      watchUrl: `https://www.youtube.com/watch?v=${item.id.videoId}`,
      thumbnail: item.snippet.thumbnails.default.url,
      viewCount: parseInt(viewCounts[item.id.videoId], 10) || 0,
    }));

    // Sort results by view count in descending order
    resultsWithViewCount.sort((a, b) => b.viewCount - a.viewCount);

    return resultsWithViewCount;
  } catch (error) {
    // Handle errors
    console.error(error);
    return {
      error: 'Failed to fetch search results',
    };
  }
});