// lib/Search.ts

export default class Search {
  static async performSearch(searchTerm: string): Promise<any[]> {
    if (!searchTerm) {
      throw new Error('No search term provided');
    }

    try {
      const response = await fetch(`https://itunes.apple.com/search?term=${encodeURIComponent(searchTerm)}&limit=20`);
      const data = await response.json();
      const searchResults = data.results;

      if (!searchResults || searchResults.length === 0) {
        return [];
      }

      return searchResults;
    } catch (error) {
      console.error('Failed to perform search:', error);
      throw error;
    }
  }
}