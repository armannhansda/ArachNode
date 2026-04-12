import axios from "axios";

export const fetchSearchResult = async (q: string, page: number) => {
  const res = await axios.get("http://localhost:3001/search", {params :{q, page, limit:10}});
  return res.data;

};

export const fetchSuggestions = async (q:string) => {
  const res = await axios.get("http://localhost:3001/suggetion", {
    params: {q},
  });
  return res.data;
};
