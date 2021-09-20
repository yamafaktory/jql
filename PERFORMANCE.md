| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 41.9 ± 2.2 | 39.4 | 57.2 | 19.78 ± 7.59 |
| `echo '[1, [2], [[3]]]' \| jql '...'` | 2.1 ± 0.8 | 1.6 | 14.6 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 41.6 ± 2.3 | 39.1 | 60.9 | 21.53 ± 8.22 |
| `echo '[1, 2, 3]' \| jql '.[0]'` | 1.9 ± 0.7 | 1.5 | 13.2 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 41.8 ± 2.1 | 39.3 | 59.3 | 21.19 ± 9.84 |
| `echo '{ "foo": "bar" }' \| jql '."foo"'` | 2.0 ± 0.9 | 1.5 | 15.8 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 195.6 ± 5.9 | 186.7 | 232.0 | 1.30 ± 0.07 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '.\|{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 150.6 ± 6.1 | 140.9 | 188.8 | 1.00 |

