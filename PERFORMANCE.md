| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 42.2 ± 3.0 | 38.2 | 70.0 | 16.81 ± 7.16 |
| `echo '[1, [2], [[3]]]' \| jql '...'` | 2.5 ± 1.1 | 1.8 | 14.3 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 42.0 ± 3.2 | 37.9 | 61.1 | 17.62 ± 7.45 |
| `echo '[1, 2, 3]' \| jql '.[0]'` | 2.4 ± 1.0 | 1.6 | 12.0 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 42.5 ± 3.1 | 38.0 | 64.2 | 16.18 ± 6.67 |
| `echo '{ "foo": "bar" }' \| jql '."foo"'` | 2.6 ± 1.1 | 1.8 | 13.0 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 208.3 ± 11.6 | 195.1 | 260.4 | 1.25 ± 0.10 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '.\|{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 166.2 ± 9.4 | 153.8 | 239.6 | 1.00 |

