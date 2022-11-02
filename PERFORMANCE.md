| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 37.9 ± 2.7 | 30.6 | 50.1 | 14.95 ± 5.06 |
| `echo '[1, [2], [[3]]]' \| jql '...'` | 2.5 ± 0.8 | 1.3 | 19.0 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 38.8 ± 2.2 | 32.3 | 52.0 | 14.79 ± 5.83 |
| `echo '[1, 2, 3]' \| jql '.[0]'` | 2.6 ± 1.0 | 1.6 | 23.2 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 39.7 ± 2.7 | 32.6 | 64.4 | 14.25 ± 4.42 |
| `echo '{ "foo": "bar" }' \| jql '."foo"'` | 2.8 ± 0.8 | 1.7 | 10.7 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 194.3 ± 8.7 | 166.7 | 240.0 | 1.39 ± 0.09 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '.\|{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 140.0 ± 7.1 | 122.2 | 179.3 | 1.00 |

