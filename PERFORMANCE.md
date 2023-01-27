| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 34.9 ± 2.6 | 30.7 | 47.4 | 10.36 ± 5.87 |
| `echo '[1, [2], [[3]]]' \| jql '...'` | 3.4 ± 1.9 | 2.1 | 28.7 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 34.3 ± 2.9 | 30.2 | 62.8 | 12.71 ± 7.59 |
| `echo '[1, 2, 3]' \| jql '.[0]'` | 2.7 ± 1.6 | 1.6 | 27.7 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 34.4 ± 2.5 | 30.4 | 47.6 | 11.39 ± 5.39 |
| `echo '{ "foo": "bar" }' \| jql '."foo"'` | 3.0 ± 1.4 | 1.8 | 19.6 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 207.2 ± 12.2 | 181.0 | 275.6 | 1.20 ± 0.11 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '.\|{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 172.9 ± 11.4 | 143.1 | 228.8 | 1.00 |

