| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 43.3 ± 2.2 | 37.9 | 58.6 | 17.45 ± 6.43 |
| `echo '[1, [2], [[3]]]' \| jql '...'` | 2.5 ± 0.9 | 1.5 | 21.3 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 43.3 ± 2.2 | 36.7 | 56.7 | 17.97 ± 5.13 |
| `echo '[1, 2, 3]' \| jql '.[0]'` | 2.4 ± 0.7 | 1.5 | 11.5 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 42.9 ± 2.2 | 37.4 | 53.9 | 17.66 ± 4.36 |
| `echo '{ "foo": "bar" }' \| jql '."foo"'` | 2.4 ± 0.6 | 1.4 | 9.7 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 192.8 ± 9.2 | 173.3 | 342.3 | 1.34 ± 0.09 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '.\|{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 144.1 ± 7.0 | 128.3 | 181.9 | 1.00 |

