export type Project = {
  title: string;
  link: string;
  description: string;
};

const hobbies: Project[] = [
  {
    title: "Zenn",
    link: "https://zenn.dev/467",
    description: "technology blog",
  },
  {
    title: "Filmarks",
    link: "https://filmarks.com/users/467",
    description: "movies",
  },
  {
    title: "Hatena",
    link: "https://s6n.hatenablog.com/",
    description: "travels",
  }
];

export default hobbies;
