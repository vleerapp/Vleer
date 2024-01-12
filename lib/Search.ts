// lib/Search.ts

import axios from 'axios';

export default class Search {
  static async performSearch(searchTerm: string): Promise<any[]> {
    if (!searchTerm) {
      throw new Error('No search term provided');
    }

    try {
      const wirewaySearchResponse = await axios.get(`https://wireway.ch/api/musicAPI/search/?q=${searchTerm}`);
      const searchResults = wirewaySearchResponse.data;

      if (!searchResults || searchResults.length === 0) {
        return [];
      }

      // For each search result, fetch additional metadata
      const metadataPromises = searchResults.map(result =>
        axios.get(`https://itunes.apple.com/search?term=${encodeURIComponent(result.title)}`)
      );

      const metadataResponses = await Promise.all(metadataPromises);

      // Extract the first result from each metadata response
      const metadata = metadataResponses.map(response => response.data.results[0]);

      return metadata;
    } catch (error) {
      console.error('Failed to perform search:', error);
      throw error;
    }
  }
}